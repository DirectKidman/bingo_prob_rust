//! このプログラムはビンゴの確率を求めるものです。
//! ## 詳細
//! ｎ個の数字が出た状態でビンゴがでる確率(%)が表示されます。
//! これは１ビンゴ以上でた確率なので、ｎ回目に初めてビンゴが出る確率ではありません。いずれ実装しますが。

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
