//! 呼び出し順を全通り平均したときの「n 回目までにビンゴする確率」と
//! 「ちょうど n 回目に初めてビンゴする確率」を表示する。
//!
//! ```bash
//! cargo run --example forecast --release              # ゲーム開始時点から
//! cargo run --example forecast --release -- 7 22 38  # 7, 22, 38 を呼んだ状態から
//! ```

use bingo::InclusionExclusionSolver;

fn main() {
    let called: Vec<u128> = std::env::args()
        .skip(1)
        .map(|s| s.parse().expect("numbers must be integers in 1..=75"))
        .collect();

    let mut solver = InclusionExclusionSolver::new(5, 15);
    for &n in &called {
        solver.play(n);
    }

    let start = called.len();
    let cumulative = solver.forecast();
    let mut expected = 0.0;
    println!("turn   by turn (%)   first bingo at turn (%)");
    for (i, &p) in cumulative.iter().enumerate() {
        let first = if i == 0 { p } else { p - cumulative[i - 1] };
        if i > 0 {
            expected += (start + i) as f64 * first / 100.0;
        }
        println!("{:>4}   {:>11.6}   {:>11.6}", start + i, p, first);
    }
    println!(
        "expected first-bingo turn (cards without bingo now): {:.3}",
        expected / (1.0 - cumulative[0] / 100.0)
    );
}
