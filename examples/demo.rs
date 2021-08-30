use bingo::solver::faster::FasterBingo;
use bingo::solver::bingo::Bingo;

fn main() {
    // let mut results = vec![];

    let numbers: Vec<usize> = (1..=75).into_iter().map(|x| x * 11 % 76).collect();
    let res1 = bingo_prob(&numbers);
    println!("{:?}", res1);
    // results.push(result);

    // let numbers: Vec<usize> = (1..=75).into_iter().collect();
    // let result = bingo_prob(&numbers);
    // results.push(result);

    // let mut rng = rand::thread_rng();
    // let mut numbers: Vec<usize> = (1..=75).into_iter().collect();
    // numbers.shuffle(&mut rng);
    // let result = bingo_prob(&numbers);
    // results.push(result);

    // for i in 0..75 {
    //     println!(
    //         "{} {} {} {}",
    //         i + 1,
    //         results[0][i],
    //         results[1][i],
    //         results[2][i]
    //     );
    // }
    let mut b = FasterBingo::new(5, 15);
    for n in numbers {
        b.play(n);
    }

    let res2 = b.get_bingo_cards();
    println!("{:?}", res2);
    println!("res1 == res2 ? :{}", res1 == res2);
}

fn bingo_prob(numbers: &[usize]) -> Vec<u128> {
    let mut b = Bingo::new(5, 15);
    b.init();

    let res = (0..75)
        .into_iter()
        .map(|x| b.play(numbers[x] as u128))
        .collect();
    res
}
