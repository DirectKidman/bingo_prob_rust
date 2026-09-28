//! ビンゴ確率のソルバー。
//!
//! - [`CellSearchSolver`] : 埋まったマスのパターンを bit 全探索する素直な実装。遅いが検算用。
//! - [`InclusionExclusionSolver`] : ビンゴラインの組み合わせに包除原理を適用する高速な実装。

mod cell_search;
mod inclusion_exclusion;

pub use cell_search::CellSearchSolver;
pub use inclusion_exclusion::InclusionExclusionSolver;

fn to_percentages(cards: &[u128], all_cards: u128) -> Vec<f64> {
    cards
        .iter()
        .map(|&c| c as f64 * 100.0 / all_cards as f64)
        .collect()
}
