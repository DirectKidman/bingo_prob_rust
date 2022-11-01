use crate::util::Permutation;

// This struct is for solving probability of bingos.
// bingo_size: the number of a edge.
//
pub struct FasterBingo {
    bingo_size: u128,
    bingo_range: u128,
    necessary_column_cells: Vec<Vec<Vec<usize>>>,
    num_called_column: Vec<usize>,

    all_cards: u128,
    bingo_cards_per_each_turn: Vec<u128>,
    called_number: Vec<usize>,

    perm_gen: Permutation,
}

impl FasterBingo {
    pub fn new(bingo_size: u128, bingo_range: u128) -> Self {
        let necessary_column_cells = vec![vec![]; 2 * bingo_size as usize + 3];
        let num_called_column = vec![0; bingo_size as usize];
        let perm_gen = Permutation::new(bingo_range, bingo_size);

        let all_cards =
            (perm_gen.perm(bingo_range, bingo_size)).pow(5u32) / (bingo_range - bingo_size + 1);
        let bingo_cards_per_each_turn = vec![];
        let called_number = vec![];

        let mut faster_bingo = FasterBingo {
            bingo_size,
            bingo_range,
            necessary_column_cells,
            num_called_column,
            all_cards,
            bingo_cards_per_each_turn,
            called_number,
            perm_gen,
        };

        faster_bingo.init();
        faster_bingo
    }

    pub fn init(&mut self) {
        let column_mask = (1 << self.bingo_size) - 1;
        let raw_mask = (1 << (2 * self.bingo_size)) - (1 << self.bingo_size);
        let cross_mask = !(column_mask + raw_mask);

        for bit_bingos in 0..(1u128 << (2 * self.bingo_size + 2)) {
            let column = bit_bingos & column_mask;
            let raw = (bit_bingos & raw_mask) >> self.bingo_size;
            let cross = (bit_bingos & cross_mask) >> (2 * self.bingo_size);

            // println!("{:0b} {:0b} {:0b} {:0b}", bit_bingos, cross, column, raw);

            let mut column_cells = vec![0; self.bingo_size as usize];
            for bit in 0..self.bingo_size {
                if (column >> bit) & 1 == 1 {
                    column_cells[bit as usize] = if bit == self.bingo_size / 2 { 4 } else { 5 };
                }
            }

            for bit in 0..self.bingo_size {
                if (raw >> bit) & 1 == 1 {
                    for c in 0..self.bingo_size {
                        if c == 2 && bit == 2 {
                            continue;
                        }

                        if (column >> c) & 1 == 0 {
                            column_cells[c as usize] += 1;
                        }
                    }
                }
            }

            for bit in 0..2 {
                if (cross >> bit) & 1 == 1 {
                    let mask = match bit {
                        0 => !(column | raw | 1 << (self.bingo_size / 2)),
                        _ => {
                            !(column
                                | raw.reverse_bits() >> (128 - self.bingo_size)
                                | 1 << (self.bingo_size / 2))
                        }
                    };

                    for c in 0..self.bingo_size {
                        if (mask >> c) & 1 == 1 {
                            column_cells[c as usize] += 1;
                        }
                    }
                }
            }

            let num_bingo = bit_bingos.count_ones();
            self.necessary_column_cells[num_bingo as usize].push(column_cells);
        }
    }

    pub fn count_bingo_cards(&mut self) -> u128 {
        let mut bingo_cards = 0i128;

        for n_bingo in 1..=(2 * self.bingo_size + 2) {
            for column_cells in &self.necessary_column_cells[n_bingo as usize] {
                let mut tmp_cards = 1u128;
                let mut can_calc = true;
                for i in 0..self.bingo_size {
                    let called = self.num_called_column[i as usize] as u128;
                    let needed = column_cells[i as usize] as u128;
                    if needed > called {
                        can_calc = false;
                        break;
                    }

                    if i == self.bingo_size / 2 {
                        tmp_cards *= self.perm_gen.perm(called, needed)
                            * self
                                .perm_gen
                                .perm(self.bingo_range - needed, self.bingo_size - needed - 1);
                    } else {
                        tmp_cards *= self.perm_gen.perm(called, needed)
                            * self
                                .perm_gen
                                .perm(self.bingo_range - needed, self.bingo_size - needed);
                    }
                }

                if can_calc {
                    // println!("{} {} {}", bingo_cards, tmp_cards, n_bingo);
                    match n_bingo % 2 {
                        1 => bingo_cards += tmp_cards as i128,
                        _ => bingo_cards -= tmp_cards as i128,
                    }
                }
            }
        }
        if bingo_cards < 0 {
            panic!("Why bingo cards became minus??");
        }
        self.bingo_cards_per_each_turn.push(bingo_cards as u128);
        bingo_cards as u128
    }

    fn add_card(&mut self, called_number: usize) {
        self.num_called_column[(called_number - 1) / self.bingo_range as usize] += 1;
        self.called_number.push(called_number);
    }

    pub fn play(&mut self, called_number: usize) -> u128 {
        self.add_card(called_number);
        let res = self.count_bingo_cards();
        res
    }

    pub fn get_probabilities(&self) -> Vec<f64> {
        let res = self
            .bingo_cards_per_each_turn
            .iter()
            .cloned()
            .map(|x| (x as f64) * 100f64 / (self.all_cards as f64))
            .collect();
        res
    }

    pub fn get_bingo_cards(&self) -> Vec<u128> {
        self.bingo_cards_per_each_turn.iter().cloned().collect()
    }
}
