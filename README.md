# ProgramingContest

## 勉強法

- ABC042から最新までの合計292問を解く
  - 気になる問題は蟻本確認
- 下記項目に当てはまるものは
  - 使い時
  - 考え方、などをメモ
- [AtCoder problems](https://kenkoooo.com/atcoder/#/table/)

## 項目

- 全探索・全列挙
  - 順列の全列挙(next_permutationを使うと楽)
  - 2nや3n通りの全列挙(bit演算でも関数を使った列挙でも)
- 木の基本的な性質(辺の数がN-1になる事とか二点間の道が必ず一つな事とか、基本的な事だけ)
- 深さ優先探索、幅優先探索(特にグリッド上とグラフ上での探索)
- 累積和
- 動的計画法
  - 漸化式を使った簡単なDP(確率DPとか桁DPは必須じゃないです)
- 素数判定
  - 約数計算や素因数分解、エラストテネスの篩 
- 最短経路問題(ワーシャルフロイド法、ダイクストラ法)
  - ワーシャルフロイド法
  - ダイクストラ法
- 二部探索
  - upper_bound, lower_bound使ったやつ
- メモ化再帰(重複計算を避けて計算量を落とす)
- 最小公倍数、最大公約数の計算(ライブラリとして持っておくと楽)
- 除算を除く四則演算のMOD計算
- Union-Find木(なくても解けるけどあると簡単になる問題がかなり多い印象)

### その他

- 倍数判定

# 参考

- [AtCoderコンテストにRustで参加するためのガイドブック](https://doc.rust-jp.rs/atcoder-rust-resources/)
- [AtCoer 過去問精選10問](https://qiita.com/drken/items/fd4e5e3630d0f5859067)
- [AtCoder に登録したら解くべき精選過去問 10 問を Rust で解いてみた](https://qiita.com/tubo28/items/e6076e9040da57368845)
- [リファレンス](https://cpprefjp.github.io/reference.html)
- [Rustで競技プログラミング スターターキット](https://qiita.com/hatoo@github/items/fa14ad36a1b568d14f3e)
- [AtCoder と Rust で始める！競技プログラミング入門（Rust 版 APG4b）](https://zenn.dev/toga/books/apg4b-rust-ver)
- [今日中にRustで10000点分ときます(提出ぶら下げ用ツイート)](https://twitter.com/saba_kpr/status/1208039505690681344)
- [Rustに入門したのでとりあえずAtCoderで1万点分解いてみた](https://saba-kpr.hatenablog.com/entry/2019/12/21/232710)
- [便利メソッドを生やす - データ構造 謹製コピペコード](https://yoshrc.github.io/rust-atcoder-snippets/atcoder_snippets/index.html)