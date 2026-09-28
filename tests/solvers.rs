use bingo::{CellSearchSolver, InclusionExclusionSolver};

/// `1..=size*range` を `step` 刻みで並べ替えた呼び出し順 (`step` と `size*range+1` は互いに素にすること)。
fn call_order(size: u128, range: u128, step: u128) -> Vec<u128> {
    let total = size * range;
    (1..=total).map(|x| x * step % (total + 1)).collect()
}

fn run_both(size: u128, range: u128, numbers: &[u128]) -> (Vec<u128>, Vec<u128>) {
    let mut fast = InclusionExclusionSolver::new(size, range);
    let mut slow = CellSearchSolver::new(size, range);
    for &n in numbers {
        fast.play(n);
        slow.play(n);
    }
    assert_eq!(fast.all_cards(), slow.all_cards());
    (fast.bingo_cards().to_vec(), slow.bingo_cards().to_vec())
}

#[test]
fn solvers_agree_on_3x3() {
    for range in [3, 4, 6] {
        let total = 3 * range;
        for step in (1..=total).filter(|s| gcd(*s, total + 1) == 1) {
            let numbers = call_order(3, range, step);
            let (fast, slow) = run_both(3, range, &numbers);
            assert_eq!(fast, slow, "range={} order={:?}", range, numbers);
        }
    }
}

#[test]
fn solvers_agree_on_1x1() {
    let (fast, slow) = run_both(1, 3, &[2, 1, 3]);
    assert_eq!(fast, slow);
    // 1x1 はフリーマス 1 つだけのカード 1 種類で、最初からビンゴ。
    assert_eq!(fast, vec![1, 1, 1]);
}

#[test]
fn all_cards_count() {
    // 5x5, 各列 15 個: P(15,5)^4 * P(15,4)
    assert_eq!(
        InclusionExclusionSolver::new(5, 15).all_cards(),
        360_360u128.pow(4) * 32_760
    );
}

#[test]
fn probability_is_monotonic_and_reaches_100_percent() {
    let mut solver = InclusionExclusionSolver::new(5, 15);
    for n in call_order(5, 15, 11) {
        solver.play(n);
    }
    let cards = solver.bingo_cards();
    assert!(cards.windows(2).all(|w| w[0] <= w[1]));
    assert_eq!(*cards.last().unwrap(), solver.all_cards());
    assert_eq!(*solver.probabilities().last().unwrap(), 100.0);
}

#[test]
fn no_bingo_before_four_calls_on_5x5() {
    // フリーマスを通るラインでも 4 マス必要。
    let mut solver = InclusionExclusionSolver::new(5, 15);
    for n in [1, 16, 46, 61] {
        solver.play(n);
    }
    assert_eq!(&solver.bingo_cards()[..3], &[0, 0, 0]);
    assert!(solver.bingo_cards()[3] > 0);
}

#[test]
#[should_panic(expected = "out of range")]
fn play_rejects_out_of_range_number() {
    InclusionExclusionSolver::new(5, 15).play(76);
}

#[test]
#[should_panic(expected = "size must be odd")]
fn new_rejects_even_size() {
    InclusionExclusionSolver::new(4, 15);
}

/// 5x5 での突き合わせ。`CellSearchSolver` の初期化が重いので通常は実行しない:
/// `cargo test --release -- --ignored`
#[test]
#[ignore]
fn solvers_agree_on_5x5() {
    let (fast, slow) = run_both(5, 15, &call_order(5, 15, 11));
    assert_eq!(fast, slow);
}

fn gcd(a: u128, b: u128) -> u128 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}
