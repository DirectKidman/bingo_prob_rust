//! 呼び出し順を全通り平均したときの「n 回目までにビンゴする確率」と
//! 「ちょうど n 回目に初めてビンゴする確率」を表示する。
//! カードは size x size、各列の数字は 3*size 個 (5x5 なら 1〜75)。
//!
//! ```bash
//! cargo run --example forecast --release                      # 5x5、ゲーム開始時点から
//! cargo run --example forecast --release -- 7 22 38           # 7, 22, 38 を呼んだ状態から
//! cargo run --example forecast --release -- --size 7          # 7x7 (各列 21 個)
//! ```

use bingo::ColumnDpSolver;

fn main() {
    let mut args = std::env::args().skip(1).peekable();
    let mut size = 5;
    if args.peek().map(String::as_str) == Some("--size") {
        args.next();
        size = args
            .next()
            .and_then(|s| s.parse().ok())
            .expect("--size needs an odd number");
    }
    let range = 3 * size;
    let called: Vec<u128> = args
        .map(|s| s.parse().expect("numbers must be integers"))
        .collect();

    let mut solver = ColumnDpSolver::new(size, range);
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
