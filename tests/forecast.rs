use bingo::InclusionExclusionSolver;
use num_bigint::BigInt;
use num_rational::BigRational;

fn ratio(numer: u128, denom: u128) -> BigRational {
    BigRational::new(BigInt::from(numer), BigInt::from(denom))
}

fn binomial(n: u128, k: u128) -> u128 {
    (0..k).fold(1, |acc, i| acc * (n - i) / (i + 1))
}

/// 総当たり: 呼び終えた `called` に、残りの数字の部分集合を全通り足して既存ソルバーで数え、
/// 追加個数ごとに平均する。(k 個呼んだ時点の集合は全部分集合が等確率なので、全呼び出し順の平均と同じ。)
fn brute_force(size: u128, range: u128, called: &[u128]) -> Vec<BigRational> {
    let rest: Vec<u128> = (1..=size * range).filter(|x| !called.contains(x)).collect();
    let mut sums = vec![0u128; rest.len() + 1];
    let mut all_cards = 0;
    for subset in 0u32..(1 << rest.len()) {
        let mut solver = InclusionExclusionSolver::new(size, range);
        all_cards = solver.all_cards();
        let picked = rest
            .iter()
            .enumerate()
            .filter(|(i, _)| (subset >> i) & 1 == 1);
        let mut cards = if size == 1 { 1 } else { 0 };
        for &n in called.iter().chain(picked.map(|(_, n)| n)) {
            cards = solver.play(n);
        }
        sums[subset.count_ones() as usize] += cards;
    }
    sums.iter()
        .enumerate()
        .map(|(j, &s)| ratio(s, binomial(rest.len() as u128, j as u128) * all_cards))
        .collect()
}

fn forecast_after(size: u128, range: u128, called: &[u128]) -> Vec<BigRational> {
    let mut solver = InclusionExclusionSolver::new(size, range);
    for &n in called {
        solver.play(n);
    }
    solver.forecast_exact()
}

#[test]
fn matches_brute_force_from_start() {
    for (size, range) in [(1, 3), (3, 3), (3, 4)] {
        assert_eq!(
            forecast_after(size, range, &[]),
            brute_force(size, range, &[]),
            "size={} range={}",
            size,
            range
        );
    }
}

#[test]
fn matches_brute_force_from_partial_state() {
    // 3x3, 各列 4 個 (列 1: 1-4, 列 2: 5-8, 列 3: 9-12)
    for called in [
        vec![1],
        vec![5],
        vec![1, 2, 3],
        vec![5, 6, 1, 12],
        vec![1, 2, 3, 4, 9],
    ] {
        assert_eq!(
            forecast_after(3, 4, &called),
            brute_force(3, 4, &called),
            "called={:?}",
            called
        );
    }
}

#[test]
fn exact_values_on_5x5_from_start() {
    let p = forecast_after(5, 15, &[]);
    assert_eq!(p.len(), 76);
    assert_eq!(p[3], ratio(0, 1));
    // 4 個で揃うのはフリーマスを通る 4 ラインだけ。
    assert_eq!(p[4], ratio(4, binomial(75, 4)));
    // 5 個: 4 ラインのどれか + 他の 51 個から 1 個 (4*51)、または 5 マスのライン 8 本と 4 ライン + カード上の他マス (8 + 4*20)
    assert_eq!(p[5], ratio(4 * 51 + 8 + 4 * 20, binomial(75, 5)));
    assert_eq!(p[75], ratio(1, 1));
}

#[test]
fn partial_state_is_consistent_with_play() {
    let mut solver = InclusionExclusionSolver::new(5, 15);
    for n in [1, 16, 31, 46, 61, 2, 17, 47] {
        solver.play(n);
    }
    let p = solver.forecast_exact();
    assert_eq!(p.len(), 75 - 8 + 1);
    // 先頭は現在の確率そのもの。
    assert_eq!(
        p[0],
        ratio(*solver.bingo_cards().last().unwrap(), solver.all_cards())
    );
    assert!(p.windows(2).all(|w| w[0] <= w[1]));
    assert_eq!(*p.last().unwrap(), ratio(1, 1));
}
