//! 2 つのソルバーで同じ順に数字を呼び、結果が一致するか確認するデモ。
//!
//! `CellSearchSolver` の初期化が重いので release ビルドで実行すること:
//! ```bash
//! cargo run --example demo --release
//! ```

use bingo::{CellSearchSolver, InclusionExclusionSolver};

fn main() {
    // 1..=75 を適当に並べ替えた呼び出し順。
    let numbers: Vec<u128> = (1..=75).map(|x| x * 11 % 76).collect();

    let mut fast = InclusionExclusionSolver::new(5, 15);
    let mut slow = CellSearchSolver::new(5, 15);
    for &n in &numbers {
        fast.play(n);
        slow.play(n);
    }

    for (turn, (n, p)) in numbers.iter().zip(fast.probabilities()).enumerate() {
        println!("{:>2}: call {:>2} -> {:.6}%", turn + 1, n, p);
    }
    println!("same result: {}", fast.bingo_cards() == slow.bingo_cards());
}
