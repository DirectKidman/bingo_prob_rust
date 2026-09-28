//! 順列計算のユーティリティ。

/// 順列 `P(n, r) = n! / (n - r)!` を計算する。
///
/// # Panics
/// `n < r` のとき。
pub fn permutation(n: u128, r: u128) -> u128 {
    assert!(n >= r, "permutation: n ({}) must be >= r ({})", n, r);
    (0..r).map(|i| n - i).product()
}

/// `P(n, r)` (`n <= max_n`, `r <= max_r`) を事前計算したテーブル。
/// `n < r` のときは 0 を返す。
pub struct PermutationTable {
    table: Vec<Vec<u128>>,
}

impl PermutationTable {
    pub fn new(max_n: u128, max_r: u128) -> Self {
        let mut table = vec![vec![0u128; max_r as usize + 1]; max_n as usize + 1];
        for (n, row) in table.iter_mut().enumerate() {
            row[0] = 1;
            for r in 1..=(max_r as usize).min(n) {
                row[r] = (n - r + 1) as u128 * row[r - 1];
            }
        }
        PermutationTable { table }
    }

    pub fn get(&self, n: u128, r: u128) -> u128 {
        self.table[n as usize][r as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn permutation_values() {
        assert_eq!(permutation(15, 5), 360_360);
        assert_eq!(permutation(5, 0), 1);
        assert_eq!(permutation(5, 5), 120);
        assert_eq!(permutation(0, 0), 1);
    }

    #[test]
    #[should_panic]
    fn permutation_panics_when_r_exceeds_n() {
        permutation(3, 4);
    }

    #[test]
    fn table_matches_permutation() {
        let table = PermutationTable::new(15, 5);
        for n in 0..=15 {
            for r in 0..=5 {
                let expected = if n >= r { permutation(n, r) } else { 0 };
                assert_eq!(table.get(n, r), expected, "P({}, {})", n, r);
            }
        }
    }
}
