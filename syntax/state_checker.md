# StateChecker
パラダイムアイデア
FoViaにおいての型理論では型に状態を持たせるアイデアを用いる。広義ではTypestateに分類されると思う。
具体的な問題解決として
- 最適化分野
  - 暗黙の境界チェックをコンパイル時にすべて排除
  - 完璧で効率的なCompile-time Evaluation(コンパイル時計算)
  - Partial Evaluation(分部評価)
  - メモリの追跡によるメモリ自動管理
- 安全面
  - Precondition Violation(事前条件違反)の排除
  - メモリ安全
  - UBの低減

# 実装
一般にRefinement Typesに分類されるとおもう
具体的にFoViaではプリミティブ型のすべてに集合論で集合Xの部分集合に属するか判別する演算を持つ

$$ X: \mathbf{PrimitiveType} \quad\text{and}\quad (X ; \subseteq)  \in \mathbf{Poset} $$

半順序集合としてよいよね。。頭まわらん。

i32は以下のように定義できる

$$ \mathbf{i32}=\{n \in Z∣−231 \leq n \leq 231−1\} $$

で、FoViaのプリミティブ型には前述の演算が定義されているので、0より小さい変数xは
$$ \mathbf{i32}_{neg} $$


