//! ビンゴラインの組み合わせに包除原理を適用するソルバー。
//!
//! - 初期化: `O(2^(2n+2) * n)` — ビンゴライン (2n+2 本) の部分集合を全列挙し、
//!   それらを全て揃えるのに各列で何マス必要かを求める。
//! - 1 手ごと: `O(4^n * n)` — 包除原理で「1 ライン以上揃っているカード」の枚数を数える。

use super::to_percentages;
use crate::math::PermutationTable;

pub struct InclusionExclusionSolver {
    size: u128,
    range: u128,
    all_cards: u128,
    /// `required_cells[k]`: ちょうど `k` 本のラインの組それぞれについて、
    /// 全て揃えるのに各列で必要な (フリーマス以外の) マス数。
    required_cells: Vec<Vec<Vec<u128>>>,
    /// 各列で呼ばれた数字の個数。
    called_per_column: Vec<u128>,
    /// 各ターン終了時点でビンゴしているカードの枚数。
    bingo_cards: Vec<u128>,
    perm: PermutationTable,
}

impl InclusionExclusionSolver {
    /// # Panics
    /// `size` が偶数のとき、または `range < size` のとき。
    pub fn new(size: u128, range: u128) -> Self {
        assert!(size % 2 == 1, "size must be odd (the center cell is free)");
        assert!(range >= size, "range must be >= size");

        let perm = PermutationTable::new(range, size);
        let all_cards = perm.get(range, size).pow(size as u32 - 1) * perm.get(range, size - 1);

        let mut solver = InclusionExclusionSolver {
            size,
            range,
            all_cards,
            required_cells: vec![],
            called_per_column: vec![0; size as usize],
            bingo_cards: vec![],
            perm,
        };
        solver.required_cells = solver.enumerate_line_sets();
        solver
    }

    /// ライン集合は `2n+2` ビットで表す: 下位 n ビットが縦、次の n ビットが横、上位 2 ビットが斜め。
    fn enumerate_line_sets(&self) -> Vec<Vec<Vec<u128>>> {
        let n = self.size;
        let center = n / 2;
        let column_mask = (1u128 << n) - 1;
        let row_mask = column_mask << n;
        let center_bit = 1u128 << center;

        let mut required_cells = vec![vec![]; (2 * n + 3) as usize];
        for line_set in 1..(1u128 << (2 * n + 2)) {
            let columns = line_set & column_mask;
            let rows = (line_set & row_mask) >> n;
            let diagonals = line_set >> (2 * n);

            // 縦ライン: その列を丸ごと使う。
            let mut cells: Vec<u128> = (0..n)
                .map(|c| match (columns >> c) & 1 {
                    1 if c == center => n - 1,
                    1 => n,
                    _ => 0,
                })
                .collect();

            // 横ライン: 縦ラインで数えていない列に 1 マスずつ。
            for r in (0..n).filter(|r| (rows >> r) & 1 == 1) {
                for c in (0..n).filter(|c| (columns >> c) & 1 == 0) {
                    if !(r == center && c == center) {
                        cells[c as usize] += 1;
                    }
                }
            }

            // 斜めライン: 縦・横ラインで数えていないマスに 1 マスずつ。
            // 列 c の対角マスは、主対角線なら行 c、反対角線なら行 n-1-c。
            let rows_reversed = rows.reverse_bits() >> (128 - n);
            for (d, covered_rows) in [(0, rows), (1, rows_reversed)] {
                if (diagonals >> d) & 1 == 1 {
                    let covered = columns | covered_rows | center_bit;
                    for c in (0..n).filter(|c| (covered >> c) & 1 == 0) {
                        cells[c as usize] += 1;
                    }
                }
            }

            required_cells[line_set.count_ones() as usize].push(cells);
        }
        required_cells
    }

    /// 包除原理で、現在ビンゴしているカードの枚数を数える。
    fn count_bingo_cards(&self) -> u128 {
        let n = self.size;
        let center = n / 2;
        let mut bingo_cards = 0i128;

        for (lines, line_sets) in self.required_cells.iter().enumerate() {
            for cells in line_sets {
                let mut ways = 1u128;
                for c in 0..n {
                    let called = self.called_per_column[c as usize];
                    let needed = cells[c as usize];
                    if needed > called {
                        ways = 0;
                        break;
                    }
                    let column_cells = if c == center { n - 1 } else { n };
                    ways *= self.perm.get(called, needed)
                        * self.perm.get(self.range - needed, column_cells - needed);
                }
                if lines % 2 == 1 {
                    bingo_cards += ways as i128;
                } else {
                    bingo_cards -= ways as i128;
                }
            }
        }

        assert!(
            bingo_cards >= 0,
            "inclusion-exclusion produced a negative count"
        );
        bingo_cards as u128
    }

    /// 数字 `number` (`1..=size*range`) を呼び、その時点でビンゴしているカードの枚数を返す。
    ///
    /// # Panics
    /// `number` が範囲外のとき。
    pub fn play(&mut self, number: u128) -> u128 {
        assert!(
            (1..=self.size * self.range).contains(&number),
            "number {} is out of range",
            number
        );
        self.called_per_column[((number - 1) / self.range) as usize] += 1;

        let bingo = self.count_bingo_cards();
        self.bingo_cards.push(bingo);
        bingo
    }

    /// カードの総数。
    pub fn all_cards(&self) -> u128 {
        self.all_cards
    }

    /// 各ターン終了時点でビンゴしているカードの枚数。
    pub fn bingo_cards(&self) -> &[u128] {
        &self.bingo_cards
    }

    /// 各ターン終了時点でビンゴしている確率 (%)。
    pub fn probabilities(&self) -> Vec<f64> {
        to_percentages(&self.bingo_cards, self.all_cards)
    }
}
