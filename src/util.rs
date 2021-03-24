pub fn permutation(a: u128, b: u128) -> u128 {
    if a < b {
        panic!("Error");
    }
    let mut res = 1;
    for i in 0..b {
        res *= a - i;
    }
    res
}
