use bingo::{ColumnDpSolver, InclusionExclusionSolver};
use num_bigint::BigUint;

/// `1..=size*range` を `step` 刻みで並べ替えた呼び出し順 (`step` と `size*range+1` は互いに素にすること)。
fn call_order(size: u128, range: u128, step: u128) -> Vec<u128> {
    let total = size * range;
    assert!(
        gcd(step, total + 1) == 1,
        "step must be coprime to {}",
        total + 1
    );
    (1..=total).map(|x| x * step % (total + 1)).collect()
}

fn assert_play_matches(size: u128, range: u128, numbers: &[u128]) {
    let mut dp = ColumnDpSolver::new(size, range);
    let mut ie = InclusionExclusionSolver::new(size, range);
    assert_eq!(*dp.all_cards(), BigUint::from(ie.all_cards()));
    for &n in numbers {
        assert_eq!(
            dp.play(n),
            BigUint::from(ie.play(n)),
            "size={} range={} after {:?}",
            size,
            range,
            &numbers[..dp.bingo_cards().len()]
        );
    }
}

#[test]
fn play_matches_inclusion_exclusion() {
    assert_play_matches(1, 3, &[2, 1, 3]);
    for (range, step) in [(3, 1), (3, 3), (3, 7), (5, 3), (5, 7)] {
        assert_play_matches(3, range, &call_order(3, range, step));
    }
    for step in [1, 11, 29] {
        assert_play_matches(5, 15, &call_order(5, 15, step));
    }
}

#[test]
fn play_matches_inclusion_exclusion_on_7x7() {
    // InclusionExclusionSolver は u128 の都合で 7x7 は range 9 まで。
    let numbers = call_order(7, 8, 5);
    assert_play_matches(7, 8, &numbers[..30]);
}

#[test]
fn forecast_matches_inclusion_exclusion() {
    for (size, range, called) in [
        (3, 4, vec![]),
        (3, 4, vec![1, 2, 3]),
        (3, 4, vec![5, 6, 1, 12]),
        (5, 15, vec![]),
        (5, 15, vec![1, 16, 46, 61, 2, 3, 4, 5]),
    ] {
        let mut dp = ColumnDpSolver::new(size, range);
        let mut ie = InclusionExclusionSolver::new(size, range);
        for &n in &called {
            dp.play(n);
            ie.play(n);
        }
        let exact = dp.forecast_exact();
        assert_eq!(
            exact,
            ie.forecast_exact(),
            "size={} called={:?}",
            size,
            called
        );

        // f64 版も厳密値とほぼ一致する。
        for (fast, exact) in dp.forecast().iter().zip(ie.forecast()) {
            assert!((fast - exact).abs() < 1e-9, "{} vs {}", fast, exact);
        }
    }
}

#[test]
fn works_beyond_u128_on_7x7() {
    let solver = ColumnDpSolver::new(7, 21);
    // P(21,7)^6 * P(21,6)
    let p7 = BigUint::from(586_051_200u64);
    let p6 = BigUint::from(39_070_080u64);
    assert_eq!(*solver.all_cards(), p7.pow(6) * p6);

    let p = solver.forecast();
    assert_eq!(p.len(), 148);
    assert!(p[..6].iter().all(|&x| x.abs() < 1e-12));
    // 6 個で揃うのはフリーマスを通る 4 ラインだけ: 4 / C(147, 6)
    let expected = 4.0 / 12_638_413_788.0 * 100.0;
    assert!((p[6] - expected).abs() < 1e-12, "{} vs {}", p[6], expected);
    assert!(p.windows(2).all(|w| w[0] <= w[1] + 1e-12));
    assert!((p[147] - 100.0).abs() < 1e-9);
}

fn gcd(a: u128, b: u128) -> u128 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}
