//! ビンゴの確率を求めるクレート。
//!
//! `size` × `size` のカード (中央はフリー) で、各列に `range` 個ずつの数字
//! (1 列目は `1..=range`、2 列目は `range+1..=2*range`、…) が割り当てられるとする。
//! 数字を 1 つずつ呼んでいき、その時点で 1 ビンゴ以上しているカードの枚数・確率を求める。
//! (n 回目に *初めて* ビンゴする確率ではない点に注意。)
//!
//! ## 例
//! ```rust
//! use bingo::InclusionExclusionSolver;
//!
//! let mut solver = InclusionExclusionSolver::new(5, 15);
//! for number in 1..=75 {
//!     solver.play(number);
//! }
//! // 全部の数字を呼べば、どのカードもビンゴしている。
//! assert_eq!(solver.bingo_cards().last(), Some(&solver.all_cards()));
//! ```

pub mod math;
pub mod solver;

pub use crate::solver::{CellSearchSolver, InclusionExclusionSolver};
