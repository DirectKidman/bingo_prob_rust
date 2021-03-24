pub mod bingo;
pub mod util;

#[cfg(test)]
mod tests {
    use crate::bingo::Bingo;
    use crate::util;
    #[test]
    fn perm_check() {
        println!("Test");
        assert_eq!(util::permutation(15, 5), 360360u128);
    }

    #[test]
    fn bingo_build_check() {
        let mut b = Bingo::new(5, 15);
        b.init();
        b.next(1);
        assert_eq!(1, 1);
    }
}
