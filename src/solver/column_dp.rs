//! 「ビンゴしていないカード」を列ごとの DP で数えるソルバー。
//!
//! 全カードから、どのラインも揃っていないカードを引いてビンゴしているカードを数える。
//! 列を左から 1 本ずつ決めていき、状態として
//! 「ここまで全部埋まっている行の集合」と「2 本の斜めがまだ全部埋まっているか」だけを持つ。
//! 列の重みはその列で埋まっているマスの数だけで決まるので、状態に関係ない行は個数だけで数えられ、
//! 1 列あたりの遷移は `O(3^n)` 程度になる (包除原理の `O(4^n)` より速い)。
//! 足し算と掛け算しか出てこないので打ち消し合いがなく、f64 でも精度が落ちない。
//!
//! 多倍長整数で数えるので、[`InclusionExclusionSolver`](super::InclusionExclusionSolver) と違い
//! 7x7 以上でも桁あふれしない。

use std::ops::AddAssign;

use num_bigint::{BigInt, BigUint};
use num_rational::BigRational;
use num_traits::{One, ToPrimitive, Zero};

use crate::math::binomial_row;

pub struct ColumnDpSolver {
    size: usize,
    range: u128,
    /// `perm[m][r] = P(m, r)` (`m <= range`, `r <= size`)
    perm: Vec<Vec<BigUint>>,
    all_cards: BigUint,
    /// 各列で呼ばれた数字の個数 (フリーマスは含めない)。
    called_per_column: Vec<u128>,
    /// 各ターン終了時点でビンゴしているカードの枚数。
    bingo_cards: Vec<BigUint>,
}

impl ColumnDpSolver {
    /// # Panics
    /// `size` が偶数のとき、`range < size` のとき、または `size > 16` のとき。
    pub fn new(size: u128, range: u128) -> Self {
        assert!(size % 2 == 1, "size must be odd (the center cell is free)");
        assert!(range >= size, "range must be >= size");
        assert!(size <= 16, "size must be <= 16");
        let n = size as usize;

        let perm: Vec<Vec<BigUint>> = (0..=range)
            .map(|m| {
                let mut row = vec![BigUint::one()];
                for r in 1..=size {
                    let next = if r <= m {
                        &row[r as usize - 1] * (m - r + 1)
                    } else {
                        BigUint::zero()
                    };
                    row.push(next);
                }
                row
            })
            .collect();

        let mut solver = ColumnDpSolver {
            size: n,
            range,
            perm,
            all_cards: BigUint::one(),
            called_per_column: vec![0; n],
            bingo_cards: vec![],
        };
        solver.all_cards = (0..n)
            .map(|c| solver.perm[range as usize][solver.column_cells(c)].clone())
            .product();
        solver
    }

    fn center(&self) -> usize {
        self.size / 2
    }

    /// 列 `c` の (フリーマス以外の) マス数。
    fn column_cells(&self, c: usize) -> usize {
        if c == self.center() {
            self.size - 1
        } else {
            self.size
        }
    }

    /// 列 `c` で「呼ばれている数字が乗ったマス」がちょうど特定の `s` マスになるような、
    /// 列 `c` への数字の割り当て方を、これから呼ぶ個数 `b` についての多項式で返す。
    /// (`future` が false なら今の状態だけ見るので定数多項式。)
    ///
    /// `weights[s][b] = C(残り_c, b) * P(呼ばれた数_c + b, s) * P(range - 呼ばれた数_c - b, マス数_c - s)`
    fn column_weights(&self, c: usize, future: bool) -> Vec<Vec<BigUint>> {
        let called = self.called_per_column[c];
        let binom = if future {
            binomial_row(self.range - called)
        } else {
            vec![BigUint::one()]
        };
        let cells = self.column_cells(c);
        (0..=cells)
            .map(|s| {
                binom
                    .iter()
                    .enumerate()
                    .map(|(b, w)| {
                        let v = (called + b as u128) as usize;
                        w * &self.perm[v][s] * &self.perm[self.range as usize - v][cells - s]
                    })
                    .collect()
            })
            .collect()
    }

    /// どのラインも揃っていない埋まり方について、重みの多項式を全部足したもの。
    fn count_no_bingo<T: Scalar>(&self, weights: &[Vec<Vec<T>>]) -> Vec<T> {
        let n = self.size;
        let center = self.center();
        let full = (1usize << n) - 1;
        let index = |rows: usize, d1: bool, d2: bool| rows << 2 | (d1 as usize) << 1 | d2 as usize;

        let mut dp: Vec<Option<Vec<T>>> = vec![None; 4 << n];
        dp[index(full, true, true)] = Some(vec![T::one()]);

        for (c, w) in weights.iter().enumerate() {
            // フリーマスのある中央列は、中央の行が必ず埋まっている。
            let forced = if c == center { 1usize << center } else { 0 };
            let offset = forced.count_ones() as usize;

            // with_rest[a][u] / without_full[a][u]:
            //   状態に関わる行のうち a 行が埋まっていて、関わらない u 行のうち何行埋まっていてもよいときの重みの合計。
            //   without_full は u 行全部が埋まる場合 (= 列が揃う場合) を除いたもの。
            let binom: Vec<Vec<u128>> = (0..=n as u128)
                .map(|u| (0..=u).map(|k| binomial(u, k)).collect())
                .collect();
            let mut with_rest = vec![vec![vec![]; n + 1]; n + 1];
            let mut without_full = vec![vec![vec![]; n + 1]; n + 1];
            for a in offset..=n {
                for u in 0..=(n - a) {
                    let mut all = vec![];
                    let mut partial = vec![];
                    for k in 0..=u {
                        let term = scale(&w[a + k - offset], binom[u][k]);
                        add_assign(&mut all, &term);
                        if k < u {
                            add_assign(&mut partial, &term);
                        }
                    }
                    with_rest[a][u] = all;
                    without_full[a][u] = partial;
                }
            }

            let mut next: Vec<Option<Vec<T>>> = vec![None; 4 << n];
            for (state, value) in dp.iter().enumerate() {
                let Some(value) = value else { continue };
                let rows = state >> 2;
                let d1 = (state >> 1) & 1 == 1;
                let d2 = state & 1 == 1;

                // 状態に関わる行: まだ全部埋まっている行、生きている斜めがこの列で通る行、中央列の中央の行。
                let mut tracked = rows | forced;
                if d1 {
                    tracked |= 1 << c;
                }
                if d2 {
                    tracked |= 1 << (n - 1 - c);
                }
                let untracked = n - tracked.count_ones() as usize;
                let free = tracked & !forced;

                // free の部分集合を全部たどる。
                let mut sub = free;
                loop {
                    let filled = sub | forced;
                    let a = filled.count_ones() as usize;
                    let weight = if filled == tracked {
                        &without_full[a][untracked]
                    } else {
                        &with_rest[a][untracked]
                    };
                    if !weight.is_empty() {
                        let key = index(
                            rows & filled,
                            d1 && (filled >> c) & 1 == 1,
                            d2 && (filled >> (n - 1 - c)) & 1 == 1,
                        );
                        let product = convolve(value, weight);
                        match &mut next[key] {
                            Some(acc) => add_assign(acc, &product),
                            slot => *slot = Some(product),
                        }
                    }
                    if sub == 0 {
                        break;
                    }
                    sub = (sub - 1) & free;
                }
            }
            dp = next;
        }

        dp[index(0, false, false)].take().unwrap_or_default()
    }

    /// 数字 `number` (`1..=size*range`) を呼び、その時点でビンゴしているカードの枚数を返す。
    ///
    /// # Panics
    /// `number` が範囲外のとき。
    pub fn play(&mut self, number: u128) -> BigUint {
        assert!(
            (1..=self.size as u128 * self.range).contains(&number),
            "number {} is out of range",
            number
        );
        self.called_per_column[((number - 1) / self.range) as usize] += 1;

        let weights: Vec<Vec<Vec<BigUint>>> = (0..self.size)
            .map(|c| self.column_weights(c, false))
            .collect();
        let no_bingo = self.count_no_bingo(&weights);
        let no_bingo = no_bingo.first().cloned().unwrap_or_default();
        let bingo = &self.all_cards - no_bingo;
        self.bingo_cards.push(bingo.clone());
        bingo
    }

    /// カードの総数。
    pub fn all_cards(&self) -> &BigUint {
        &self.all_cards
    }

    /// 各ターン終了時点でビンゴしているカードの枚数。
    pub fn bingo_cards(&self) -> &[BigUint] {
        &self.bingo_cards
    }

    /// 各ターン終了時点でビンゴしている確率 (%)。
    pub fn probabilities(&self) -> Vec<f64> {
        self.bingo_cards
            .iter()
            .map(|c| ratio_to_f64(c, &self.all_cards) * 100.0)
            .collect()
    }

    /// ここから先の呼び出し順を全通り平均したときの、各ターン終了時点までにビンゴしている確率 (厳密値, 0〜1)。
    ///
    /// 戻り値の `i` 番目はターン `m + i` (`m` はこれまでに呼んだ個数) の値。
    /// 意味は [`InclusionExclusionSolver::forecast_exact`](super::InclusionExclusionSolver::forecast_exact) と同じ。
    /// 多倍長整数で計算するので、大きいサイズでは [`forecast`](Self::forecast) の方がずっと速い。
    pub fn forecast_exact(&self) -> Vec<BigRational> {
        let weights: Vec<Vec<Vec<BigUint>>> = (0..self.size)
            .map(|c| self.column_weights(c, true))
            .collect();
        let no_bingo = self.count_no_bingo(&weights);

        let all_cards = BigInt::from(self.all_cards.clone());
        binomial_row(self.remaining())
            .into_iter()
            .enumerate()
            .map(|(j, orders)| {
                let total = BigInt::from(orders) * &all_cards;
                let none = no_bingo
                    .get(j)
                    .cloned()
                    .map(BigInt::from)
                    .unwrap_or_default();
                BigRational::new(&total - none, total)
            })
            .collect()
    }

    /// [`forecast_exact`](Self::forecast_exact) を f64 で計算して確率 (%) にしたもの。
    ///
    /// 各列の重みを列ごとの総数で割った確率のまま計算する。打ち消し合いがないので相対誤差は小さいが、
    /// ビンゴ確率は `1 - (ビンゴしていない確率)` で求めるため、絶対誤差は 1e-13 % 程度になる。
    pub fn forecast(&self) -> Vec<f64> {
        let weights: Vec<Vec<Vec<f64>>> = (0..self.size)
            .map(|c| {
                let column_total = &self.perm[self.range as usize][self.column_cells(c)];
                self.column_weights(c, true)
                    .iter()
                    .map(|poly| poly.iter().map(|x| ratio_to_f64(x, column_total)).collect())
                    .collect()
            })
            .collect();
        let no_bingo = self.count_no_bingo(&weights);

        binomial_row(self.remaining())
            .iter()
            .enumerate()
            .map(|(j, orders)| {
                let none = no_bingo.get(j).copied().unwrap_or(0.0);
                (1.0 - none / orders.to_f64().unwrap()) * 100.0
            })
            .collect()
    }

    /// まだ呼ばれていない数字の個数。
    fn remaining(&self) -> u128 {
        self.size as u128 * self.range - self.called_per_column.iter().sum::<u128>()
    }
}

/// DP で使う数の型 (厳密計算用の `BigUint` と、高速計算用の `f64`)。
trait Scalar: Clone + Zero + One + for<'a> AddAssign<&'a Self> {
    fn from_u128(x: u128) -> Self;
    fn mul_ref(&self, other: &Self) -> Self;
}

impl Scalar for BigUint {
    fn from_u128(x: u128) -> Self {
        BigUint::from(x)
    }
    fn mul_ref(&self, other: &Self) -> Self {
        self * other
    }
}

impl Scalar for f64 {
    fn from_u128(x: u128) -> Self {
        x as f64
    }
    fn mul_ref(&self, other: &Self) -> Self {
        self * other
    }
}

fn binomial(n: u128, k: u128) -> u128 {
    (0..k).fold(1, |acc, i| acc * (n - i) / (i + 1))
}

fn scale<T: Scalar>(poly: &[T], factor: u128) -> Vec<T> {
    let factor = T::from_u128(factor);
    poly.iter().map(|x| x.mul_ref(&factor)).collect()
}

fn add_assign<T: Scalar>(acc: &mut Vec<T>, poly: &[T]) {
    if acc.len() < poly.len() {
        acc.resize(poly.len(), T::zero());
    }
    for (a, p) in acc.iter_mut().zip(poly) {
        *a += p;
    }
}

fn convolve<T: Scalar>(a: &[T], b: &[T]) -> Vec<T> {
    let mut out = vec![T::zero(); a.len() + b.len() - 1];
    for (i, x) in a.iter().enumerate() {
        if x.is_zero() {
            continue;
        }
        for (j, y) in b.iter().enumerate() {
            out[i + j] += &x.mul_ref(y);
        }
    }
    out
}

/// 巨大な整数同士の比を f64 にする (どちらも f64 の範囲を超えても大丈夫なように)。
fn ratio_to_f64(numer: &BigUint, denom: &BigUint) -> f64 {
    BigRational::new(BigInt::from(numer.clone()), BigInt::from(denom.clone()))
        .to_f64()
        .unwrap()
}
