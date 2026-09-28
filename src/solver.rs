//! ビンゴ確率のソルバー。
//!
//! - [`CellSearchSolver`] : 埋まったマスのパターンを bit 全探索する素直な実装。遅いが検算用。
//! - [`InclusionExclusionSolver`] : ビンゴラインの組み合わせに包除原理を適用する実装。5x5 程度まで。
//! - [`ColumnDpSolver`] : ビンゴしていないカードを列ごとの DP で数える実装。いちばん速く、大きいサイズにも使える。

mod cell_search;
mod column_dp;
mod inclusion_exclusion;

pub use cell_search::CellSearchSolver;
pub use column_dp::ColumnDpSolver;
pub use inclusion_exclusion::InclusionExclusionSolver;

fn to_percentages(cards: &[u128], all_cards: u128) -> Vec<f64> {
    cards
        .iter()
        .map(|&c| c as f64 * 100.0 / all_cards as f64)
        .collect()
}
