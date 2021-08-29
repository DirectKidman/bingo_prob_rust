pub fn permutation(a: u128, b: u128) -> u128 {
    if a < b {
        panic!("Error");
    }
    let mut res = 1;
    for i in 0..b {
        res *= a - i;
    }
    res
}

pub struct Permutation {
    val: Vec<Vec<u128>>,
}

impl Permutation {
    pub fn new(n: u128, lim_r: u128) -> Self {
        let mut val = vec![vec![0u128; lim_r as usize + 1]; n as usize + 1];
        for i in 1..=n {
            for j in 0..=lim_r {
                if i < j {
                    break;
                }
                let id_i = i as usize;
                let id_j = j as usize;
                match j {
                    0 => val[id_i][id_j] = 1,
                    _ => {
                        val[id_i][id_j] = (i - j + 1) * val[id_i][id_j - 1];
                    }
                }
            }
        }
        Permutation { val: val }
    }

    pub fn perm(&self, n: u128, r: u128) -> u128 {
        self.val[n as usize][r as usize]
    }
}
