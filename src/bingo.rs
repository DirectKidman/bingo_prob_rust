use crate::util;

#[allow(dead_code)]
pub struct Bingo {
    n: u128,
    ran: u128,
    all_cards: u128,
    data: Vec<Vec<u128>>,
    called: Vec<u128>,
    called_raw: Vec<u128>,

    bingo_cards: Vec<Vec<u128>>,
}

impl Bingo {
    pub fn new(n: u128, ran: u128) -> Bingo {
        let size = (n + 1).pow(n as u32) as usize;
        let max_bingo = (n * 2 + 2) as usize;
        let mut called_raw = vec![0; n as usize];
        let all_cards = util::permutation(ran, n).pow(n as u32 - 1) * util::permutation(ran, n - 1);
        let mut bingo_cards = vec![vec![0; max_bingo]];
        bingo_cards[0][0] = all_cards;

        called_raw[(n as usize) / 2] = 1;

        Bingo {
            n,
            ran,
            all_cards,
            data: vec![vec![0; max_bingo + 1]; size],
            called: vec![],
            called_raw,
            bingo_cards,
        }
    }

    fn bit_search(&mut self) {
        let n_square = self.n * self.n;
        let mask_vec = self.make_maskvec();

        for bb in 0..(1 << n_square) {
            if (bb >> n_square / 2) & 1 == 0 {
                continue;
            }
            let mut index = 0;
            for j in 0..self.n {
                let occupied = (bb & mask_vec[j as usize]).count_ones() as u128;
                index += occupied * (self.n + 1).pow(j as u32);
            }
            let n_bingo = self.bingo_count(bb, &mask_vec);
            // println!("b: {}, i: {}, n : {}", bb, index, n_bingo);
            // println!("{} {}", index, n_bingo);
            self.data[index as usize][n_bingo] += 1;
        }
    }

    fn make_maskvec(&self) -> Vec<u128> {
        let mut mask_vec: Vec<u128> = vec![0; (self.n * 2 + 2) as usize];
        let tmp_n = self.n as usize;
        for i in 0..self.n {
            let i = i as usize;
            mask_vec[i] = (1 << tmp_n * (i + 1)) - (1 << tmp_n * i);
            for j in 0..self.n {
                let j = j as usize;
                mask_vec[self.n as usize + i] |= 1 << (tmp_n * j + i);
            }
        }

        for i in 0..self.n {
            let i = i as usize;
            let id = (self.n * 2) as usize;
            mask_vec[id] |= 1 << ((tmp_n + 1) * i);
            mask_vec[id + 1] |= 1 << ((tmp_n - 1) * i + tmp_n - 1);
        }

        mask_vec
    }

    fn bingo_count(&self, bb: u128, mask_vec: &Vec<u128>) -> usize {
        let n_bingo = mask_vec
            .iter()
            .map(|x| (x & bb).count_ones())
            .filter(|&x| x == self.n as u32)
            .count();

        n_bingo
    }

    pub fn call_number(&mut self, n: u128) {
        if n > self.n * self.ran {
            panic!("Bingo Number is out of range");
        }
        self.called.push(n);
        self.called_raw[((n - 1) / self.ran) as usize] += 1;
    }

    fn count_cards_per_n_bingo(&self, n_bingo: usize) -> u128 {
        let ptn = (self.n + 1).pow(self.n as u32);
        let mut cards = 0;
        for p in 0..ptn {
            let mut flag = true;
            let mut tmp_card = 1u128;
            let mut tmp_p = p;
            for i in 0..self.n {
                let occ = tmp_p % (self.n + 1);
                tmp_p /= self.n + 1;

                if occ > self.called_raw[i as usize]
                    || (i == self.n / 2 && occ == 0)
                    || (i != self.n / 2 && self.called_raw[i as usize] + self.n > occ + self.ran)
                    || (i == self.n / 2
                        && self.called_raw[i as usize] + self.n > occ + self.ran + 1)
                {
                    flag = false;
                    break;
                }

                // println!("tmp:{}, data:{}", tmp_card, self.data[p as usize][n_bingo]);
                if i == self.n / 2 {
                    tmp_card *= util::permutation(self.called_raw[i as usize] - 1, occ - 1)
                        * util::permutation(
                            self.ran - self.called_raw[i as usize] + 1,
                            self.n - occ,
                        );
                } else {
                    tmp_card *= util::permutation(self.called_raw[i as usize], occ)
                        * util::permutation(self.ran - self.called_raw[i as usize], self.n - occ);
                }
            }

            if flag {
                // if self.called.len() == 11 {
                //     println!("{:?}", self.called_raw);
                //     println!("p : {}", p);
                //     println!("{:?}", self.data[p as usize]);
                //     println!("{}", tmp_card);
                // }
                cards += tmp_card * self.data[p as usize][n_bingo];
            }
        }

        cards
    }

    fn count_cards(&mut self) {
        let max_bingo = (self.n * 2 + 2) as usize;
        let mut card_vec = vec![0; max_bingo + 1];
        for i in 0..=max_bingo {
            card_vec[i] = self.count_cards_per_n_bingo(i);
        }

        self.bingo_cards.push(card_vec);
    }

    fn show_bingo_probability(&self) {
        let n_turn = self.called.len();
        if n_turn == 0 {
            println!("Nothing to show");
            return;
        }

        let new_bingo_card = self.bingo_cards[n_turn - 1][0] - self.bingo_cards[n_turn][0];

        // let prob: f64 = new_bingo_card as f64 / self.all_cards as f64;
        let prob: f64 =
            (self.all_cards as f64 - self.bingo_cards[n_turn][0] as f64) / self.all_cards as f64;
        println!("Bingo Probability : {}", prob * 100f64);
    }

    pub fn init(&mut self) {
        self.bit_search();
    }

    pub fn next(&mut self, n: u128) {
        self.call_number(n);
        self.count_cards();
        self.show_bingo_probability();
    }
}
