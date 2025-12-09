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

FoViaは副作用による未確定な状態を排除するため、ユーザーにバリテーションコードを書くことを強制する。  
これでFoVia内の型は現実的に処理できる状態に制限する。  
基本的にこれらをどの程度まで自動化するかがキモだと思う。  

# 実装
一般にRefinement Typesに分類されるとおもう
具体的にFoViaではプリミティブ型のすべてに集合論で集合Xの部分集合に属するか判別する演算を持つ

$$ X: \mathbf{PrimitiveType} \quad\text{and}\quad (X ; \subseteq)  \in \mathbf{Poset} $$

半順序集合としてよいよね。。頭まわらん。


32bit 符号付き整数型 $\mathbf{i32}$ はこう

$$
    \mathbf{i32}
    = \{\, n \in \mathbb{Z} \mid -2^{31} \leq n \leq 2^{31}-1 \,\}.
$$

でi$\mathbf{i32}$はプリミティブ型なので

$$
    \mathbf{i32} : \mathbf{PrimitiveType}
    \quad\text{かつ}\quad
    (\mathbf{i32}, \subseteq) \in \mathbf{Poset}
$$

を満たす。

## 状態付きプリミティブ型

FoVia では、各プリミティブ型 $X$ に対して、その上の「状態」を表す集合 $S_X$ を取り、状態ごとに部分集合としての Refinement 型を定義する。  
ここで依存型をつかうのではなくfamilyとして表現するのはFoVia文法に落とし込んだ時の依存型と状態を混同させないため

状態付の型は以下のようにあらわせる

$$
    X_s \subseteq X
    \qquad (s \in S_X)
$$

例えば$\mathbf{i32}$ の場合、状態集合とこんなものが用意できそう

$$
    S_{\mathbf{i32}} = \{\mathrm{neg},\, \mathrm{zero},\, \mathrm{pos}\}
$$

これらを具体的に定義するとこんなかんじか

$$
\begin{align*}
    \mathbf{i32}_{\mathrm{neg}}
    &= \{\, n \in \mathbf{i32} \mid n < 0 \,\}, \\
    \mathbf{i32}_{\mathrm{zero}}
    &= \{\, n \in \mathbf{i32} \mid n = 0 \,\}, \\
    \mathbf{i32}_{\mathrm{pos}}
    &= \{\, n \in \mathbf{i32} \mid n > 0 \,\}.
\end{align*}
$$

これらはコンパイラがコードから自動で推論できるようにする。

## 状態の演算可能に
演算$\subseteq$による半順序集合なので上限下限が定義できて

$$
\begin{aligned}
s \wedge t
  &:\quad \mathbf{i32}_s \cap \mathbf{i32}_t \\[4pt]

s \vee t
  &:\quad \mathbf{i32}_s \cup \mathbf{i32}_t \\[4pt]

\end{aligned}
$$

具体的にこれをつかってぐにゃる方法は検討中

## 依存と部分集合演算

以上のように、$\mathbf{i32}$ 自身は

$$
    \mathbf{i32} : \mathbf{PrimitiveType}
$$

などで

$$
    \mathsf{Subset}(s,t) := (\mathbf{i32}_s \subseteq \mathbf{i32}_t)
$$

boolを返す関数を定義できる。

コンパイラはこの結果をもとにエラーを検知できる。

