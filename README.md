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
examples/forecast.rs             # 全呼び出し順で平均した確率の表示
tests/                           # テスト
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

## 呼び出し順を全通り平均した確率
上の2つは「決まった呼び出し順」でのビンゴ確率ですが、`InclusionExclusionSolver::forecast` を使うと、
**ここから先の呼び出し順を全通り平均したときの、各ターンまでにビンゴしている確率**を厳密に計算できます。
何も呼んでいない状態で使えばゲーム全体での「n 回目までにビンゴする確率」、途中まで `play` した状態で使えば、そこからの見込みになります。
「ちょうど n 回目に初めてビンゴする確率」は隣り合う値の差です。

考え方: 残り $R$ 個から $j$ 個呼んだ時点の呼ばれた数字の集合はどの部分集合も等確率なので、列 $c$ (残り $R_c$ 個) に新しく $b_c$ 個入る場合の数は $\prod_c \binom{R_c}{b_c}$ です。
これを包除原理の式に掛けて足し合わせると、ライン集合ごとに列ごとの多項式の積になり、その $x^j$ の係数を $\binom{R}{j} \times$ (カード総数) で割れば確率になります。
値が $10^{48}$ を超えるので多倍長整数で計算しています (5x5 で 0.05 秒程度)。

```bash
cargo run --example forecast --release              # ゲーム開始時点から
cargo run --example forecast --release -- 7 22 38  # 7, 22, 38 を呼んだ状態から
```

5x5 (各列15個) では、30 回目までにビンゴする確率が 14.4%、50 回目までで 81.4%、初めてビンゴするのは平均 41.4 回目です。

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

