//! 埋まったマスのパターンを bit 全探索するソルバー。
//!
//! - 初期化: `O(2^(n^2) * n)` — 全マスの埋まり方を列挙し、
//!   「各列で何マス埋まっているか」と「何ビンゴしているか」ごとにパターン数を数える。
//! - 1 手ごと: `O(n * (n+1)^n)` — 上で数えたパターンそれぞれについて、該当するカードの枚数を数える。
//!
//! `size = 5` の初期化は `2^25` 通りの探索になるので、release ビルドで実行すること。

use super::to_percentages;
use crate::math::permutation;

pub struct CellSearchSolver {
    size: u128,
    range: u128,
    all_cards: u128,
    /// `patterns[p][k]`: 各列の埋まったマス数を (size+1) 進数で `p` とエンコードしたとき、
    /// ちょうど `k` ビンゴしているマスの埋まり方の数。
    patterns: Vec<Vec<u128>>,
    /// 各列で呼ばれた数字の個数。中央列はフリーマスの分を 1 として数える。
    called_per_column: Vec<u128>,
    /// 各ターン終了時点でビンゴしているカードの枚数。
    bingo_cards: Vec<u128>,
}

impl CellSearchSolver {
    /// # Panics
    /// `size` が偶数のとき、または `range < size` のとき。
    pub fn new(size: u128, range: u128) -> Self {
        assert!(size % 2 == 1, "size must be odd (the center cell is free)");
        assert!(range >= size, "range must be >= size");

        let all_cards =
            permutation(range, size).pow(size as u32 - 1) * permutation(range, size - 1);
        let mut called_per_column = vec![0; size as usize];
        called_per_column[size as usize / 2] = 1;

        let mut solver = CellSearchSolver {
            size,
            range,
            all_cards,
            patterns: vec![],
            called_per_column,
            bingo_cards: vec![],
        };
        solver.patterns = solver.search_patterns();
        solver
    }

    fn max_lines(&self) -> usize {
        (2 * self.size + 2) as usize
    }

    /// 各ビンゴライン (横 n 本, 縦 n 本, 斜め 2 本) に対応するマスのビットマスク。
    /// マス `(row, col)` は bit `row * size + col` に対応する。
    fn line_masks(&self) -> Vec<u128> {
        let n = self.size as usize;
        let mut masks = vec![0u128; self.max_lines()];
        for i in 0..n {
            masks[i] = ((1 << n) - 1) << (n * i);
            for j in 0..n {
                masks[n + i] |= 1 << (n * j + i);
            }
            masks[2 * n] |= 1 << ((n + 1) * i);
            masks[2 * n + 1] |= 1 << ((n - 1) * (i + 1));
        }
        masks
    }

    fn search_patterns(&self) -> Vec<Vec<u128>> {
        let n = self.size;
        let cells = n * n;
        let center = cells / 2;
        let masks = self.line_masks();
        let columns = &masks[n as usize..2 * n as usize];

        let mut patterns = vec![vec![0; self.max_lines() + 1]; (n + 1).pow(n as u32) as usize];
        for filled in 0..(1u128 << cells) {
            if (filled >> center) & 1 == 0 {
                continue;
            }
            let index: u128 = columns
                .iter()
                .enumerate()
                .map(|(j, mask)| (filled & mask).count_ones() as u128 * (n + 1).pow(j as u32))
                .sum();
            let lines = masks.iter().filter(|&mask| filled & mask == *mask).count();
            patterns[index as usize][lines] += 1;
        }
        patterns
    }

    /// 現在の呼び出し状況で、ちょうど `lines` ビンゴしているカードの枚数。
    fn count_cards_with_lines(&self, lines: usize) -> u128 {
        let n = self.size;
        let center = n / 2;
        let mut cards = 0;
        'pattern: for p in 0..(n + 1).pow(n as u32) {
            let mut rest = p;
            let mut ways = 1u128;
            for col in 0..n {
                let filled = rest % (n + 1);
                rest /= n + 1;
                let called = self.called_per_column[col as usize];

                if col == center {
                    // フリーマスは常に埋まっていて、数字の割り当てには関与しない。
                    if filled == 0 || filled > called || called + n > filled + self.range + 1 {
                        continue 'pattern;
                    }
                    ways *= permutation(called - 1, filled - 1)
                        * permutation(self.range + 1 - called, n - filled);
                } else {
                    if filled > called || called + n > filled + self.range {
                        continue 'pattern;
                    }
                    ways *=
                        permutation(called, filled) * permutation(self.range - called, n - filled);
                }
            }
            cards += ways * self.patterns[p as usize][lines];
        }
        cards
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

        let bingo = self.all_cards - self.count_cards_with_lines(0);
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
