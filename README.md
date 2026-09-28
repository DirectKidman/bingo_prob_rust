# Bingoの確率を求めるプログラム

以前、Pythonで作ったことがあったので今回はRustで作ってみました。正直以前のPythonのプログラムがあっている自信がなかったので違う実装をして数年前のプログラムの確認をしていたり。


`src/solver/cell_search.rs` (`CellSearchSolver`) の方が、今回新しく組んだアルゴリズムで、`src/solver/inclusion_exclusion.rs` (`InclusionExclusionSolver`) の方が、Pythonからの移植のプログラムです。名前の通り後者の方が実行速度が速いです。使い道次第では前者のプログラムも使うことができるので一応残してあります。

## 構成
```
src/
├── lib.rs
├── math.rs                      # 順列計算
└── solver/
    ├── cell_search.rs           # CellSearchSolver: マスのbit全探索
    └── inclusion_exclusion.rs   # InclusionExclusionSolver: 包除原理
examples/demo.rs                 # 2つのソルバーの結果を突き合わせるデモ
tests/solvers.rs                 # テスト
```

## 計算量
n はビンゴの一辺のサイズとします。 今回は埋まってるマスをbit全探索しているのでそこがボトルネックとなって


初期化 $O(2^{n ^ 2} \cdot n)$ := 全２４マスに対してビット全探索。

一手ごと $O(n \cdot n ^ n)$ := 上で調べた全てのパターンについて、ビンゴになる枚数を調べる。

InclusionExclusionSolverの方は

初期化 $O(2 ^ n \cdot n)$ := 全１２ビンゴに対してビット全探索。

一手ごと $O(4 ^ n)$ := 包除原理を用いて、計算。


となっています。上の
前回のプログラムと実行結果は同じなのでコードがあっていること保証は高くなりました。まあRust速いのでこの計算量の違いがあってもRustのほうが実行速度上回っています笑。

## 実行
exampleにdemoを用意しましたので、そこから確認していただければ幸いです。
ちなみに、releaseビルドしない場合、遅い方のソルバーの初期化が永遠に終わりません。気をつけてください。
```bash
cargo run --example demo --release
```

また、クレートとして使用したい場合は以下のようにしていただければ使うことが可能です。crate.ioにはまだあげていません。まだ少しプログラムとして汚いので、修正次第ですかね。いつ修正するかは闇の中ですが。
```
[dependencies]
bingo = {　git = "https://github.com/DirectKidman/bingo_prob_rust", branch="master"}
```

```rust
use bingo::InclusionExclusionSolver;

let mut solver = InclusionExclusionSolver::new(5, 15); // 5x5, 各列15個 (1..=75)
solver.play(1);                                        // その時点でビンゴしているカードの枚数を返す
println!("{:?}", solver.probabilities());              // 各ターンのビンゴ確率 (%)
```

これで大丈夫です。

## テスト
```bash
cargo test
# 5x5 で2つのソルバーを突き合わせる重いテスト
cargo test --release -- --ignored
```
GUI、条件付き確率、などなど実装することはまだまだあります。Documentもコメントも全く書いてないですしおすし。
心の余裕があれば、Pythonバージョンも実装して速度比較なんかしてみたいものですね。

