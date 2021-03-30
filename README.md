# Bingoの確率を求めるプログラム

以前、Pythonで作ったことがあったので今回はRustで作ってみました。正直以前のPythonのプログラムがあっている自信がなかったので違う実装をして数年前のプログラムの確認をしていたり。

## 計算量
今回は埋まってるマスをbit全探索しているのでそこがボトルネックとなって

初期化　O(2^{n ^ n})
一手ごと　O(n ^ n)


以前のPythonのプログラムは、


初期化　O((2 ^ n) * n)
一手ごと　O(n * n)


でこっちのほうが優秀なんですよねー。汚くて見たくもないコードですが（）。
前回のプログラムと実行結果は同じなのでコードがあっていること保証は高くなりました。まあRust速いのでこの計算量の違いがあってもRustのほうが実行速度上回っています笑。

## 実行
(exampleを用意すればよかった...)
libクレートなので、これのみで実行できないんですよね。バイナリクレートにしようか迷いましたがなんとなく。

```
[dependencies]
bingo = {git = "https://github.com/DirectKidman/bingo_prob_rust", branch="master"}
```

これで大丈夫です。GUI、条件付き確率、などなど実装することはまだまだあります。
心の余裕があれば、Pythonバージョンも実装して速度比較なんかしてみたいものですね。

