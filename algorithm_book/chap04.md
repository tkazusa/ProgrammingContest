## 再帰関数のメモ化

フィボナッチ数列を求める再帰関数の例

```Python
def fibo(N):
    if  N == 0:
        return 0
    elif N == 1:
        return 1
    else return fibo(N-1) + fibo(N-2)
```

これだと、何回も fibo() を呼び出す必要があり、非効率。
例えば、下記のような形だと、一度計算したfibo(n)は再度計算する必要がない。

```Python
f = []
f[0] = 0
f[1] = 1

for i in range(2,50):
    f[i] = f[i-1] + f[i-2]

result = f[50]
```

下記のようにメモ化しておけば、一度計算したものは、再度計算する必要がなくなる。

```Python
f = []
f[0] = 0
f[1] = 1

def fibo(N):
    if  N == 0:
        return 0
    elif N == 1:
        return 1

    if (memo[N] != -1):
        return memo[N]
    
    else return fibo(N-1) + fibo(N-2)

if __name__ == '__main__':
    memo = [1]*50
    fibo(49)
```


### 部分和問題

```
N個の正の整数a0,a1,...,aN－1と正の整数Wが与えられます．
a0,a1,...,aN－1の中から何個かの整数を選んで総和をWとすることができるかどうかを判定してください．
```



# 章末問題

# 4.1

```
トリボナッチ数列とは，
- T0＝0
- T1＝0
- T2＝1
- TN＝TN－1＋TN－2＋TN－3(N＝3,4,...)

によって定義される数列です．
0,0,1,1,2,4,7,13,24,44,...と続いていきます．
トリボナッチ数列の第N項の値を求める再帰関数を設計してください．
```

コード 4.5 と同様にして、次のように実装できます。コード 4.5 ではベースケースは N = 0, 1 の 2 つの値でしたが、下の実装ではベースケースは N = 0, 1, 2 の 3 つの値となっています。

　

```cpp
#include <iostream>
using namespace std;

int func(int N) {
    // ベースケース
    if (N == 0) return 0;
    else if (N == 1) return 0;
    else if (N == 2) return 1;

    // 再帰呼び出し
    return func(N - 1) + func(N - 2) + func(N - 3);
}

int main() {
    int N;
    cin >> N;
    cout << func(N) << endl;
}
```


```Python
def func(N):
    # ベースケース
    if (N == 0): 
        return 0
    else if (N == 1):
        return 0
    else if (N == 2):
        return 1
    else:
        return func(N-1) + func(N-2) + func(N-3)

if __main__ == '__name__':
    N = 50
    result = func(N)
```
　

# 4.2

```
問題4.1 で設計した再帰関数をメモ化によって効率化してください．また，メモ化を実施後の計算量を評価してください.
```

問題 4.1 で示したコードは、指数時間の計算量となります。4.4 節で解説したメモ化によって効率化しましょう。次のように実装できます。計算量は O(N) となります。

　

```cpp
#include <iostream>
#include <vector>
using namespace std;

// fibo(N) の答えをメモ化する配列
vector<long long> memo;

long long func(int N) {
    // ベースケース
    if (N == 0) return 0;
    else if (N == 1) return 0;
    else if (N == 2) return 1;

    // メモをチェック (すでに計算済みならば答えをリターンする)
    if (memo[N] != -1) return memo[N];

    // 答えをメモ化しながら，再帰呼び出し
    return memo[N] = func(N - 1) + func(N - 2) + func(N - 3);
}

int main() {
    int N;
    cin >> N;

    // メモ化用配列を -1 で初期化する
    memo.assign(N + 1, -1);

    // fibo(N) をよびだす
    cout << func(N) << endl;
}
```

```Python
def func(N):
    # ベースケース
    if (N == 0): 
        return 0
    else if (N == 1):
        return 0
    else if (N == 2):
        return 1
    if (memo[N] != -1):
        return memo[N]
    else:
        return memo[N] = func(N-1) + func(N-2) + func(N-3)

if __main__ == '__name__':
    N = 50
    result = func(N)
```


# 4.3

ここでは省略します。次の記事でわかりやすく解説されています。ぜひ参考にしてください。


+ [フィボナッチ数列の一般項と数学的帰納法 (高校数学の美しい物語)](https://mathtrain.jp/fibonacci)


# 4.4

コード 4.5 において、fibo(N) の計算に要する時間を T(N) と表すことにすると、T(N) = T(N-1) + T(N-2) + O(1) が成立します (T(0), T(1) は定数)。よって、T(N) はフィボナッチ数列と同様の増加の仕方をします。一方、フィボナッチ数列の N 項目は

- ((1 + √5) / 2)^N
- ((1 - √5) / 2)^N

の線形和となっています。このうちの後者については、N → ∞ において 0 に収束します。したがって、T(N) = O((1 + √5) / 2)^N) と評価できます。


# 4.5 (ABC 114 C - 755)

再帰関数を用いることで、下図のように、N 以下の 753 数を列挙していくことができます。

<img src=https://github.com/drken1215/book_algorithm_solution/blob/master/fig/sol4-5.png width=450mm>

なお、「7 と 5 と 3 がすべて一度以上登場する」という条件を判定するために、

- 桁の中に 7 が存在するか
- 桁の中に 5 が存在するか
- 桁の中に 3 が存在するか

を管理するフラグ変数を用意するとよいでしょう。3 つのフラグを扱えるように、3.5 節で解説した「整数の二進法表現とビット演算」を活用するのが有効です。たとえば次のコードのように実装できます。


```cpp
#include <iostream>
using namespace std;

// N: 入力
// cur: 現在の値
// use: 7, 5, 3 のうちどれを使ったか
// counter: 答え
void func(long long N, long long cur, int use, long long &counter){
    if (cur > N) return; // ベースケース
    if (use == 0b111) ++counter; // 答えを増やす

    // 7 を付け加える
    func(N, cur * 10 + 7, use | 0b001, counter);

    // 5 を付け加える
    func(N, cur * 10 + 5, use | 0b010, counter);

    // 3 を付け加える
    func(N, cur * 10 + 3, use | 0b100, counter);
}

int main() {
    long long N;
    cin >> N;
    long long counter = 0;
    func(N, 0, 0, counter);
    cout << counter << endl;
}
```


# 4.6


```
N個の正の整数a0,a1,...,aN－1と正の整数Wが与えられます．
a0,a1,...,aN－1の中から何個かの整数を選んで総和をWとすることができるかどうかを判定してください．
部分和問題に対する再帰関数を用いる計算量O(2N)のコード4.9に対しメモ化して，O(NW)の計算量で動作するようにしてください．
```

コード 4.9 の再帰関数

 `bool func(int i, int w, const vector<int> &a)`

において、引数 i, w に対する答えを格納するメモ化用配列を次のように用意します。



- `memo[i][w]`  ← `func(i, w, a)` の答え



これを用いて、次のコードのように実装できます。計算量は O(NW) となります。

　

```cpp
#include <iostream>
#include <vector>
using namespace std;

// func(i, w, a) の答えをメモ化する配列
vector<vector<int>> memo;

// 0：false、1: true
int func(int i, int w, const vector<int> &a) {
    // ベースケース
    if (i == 0) {
        if (w == 0) return true;
        else return false;
    }

    // メモをチェック (すでに計算済みならば答えをリターンする)
    if (memo[i][w] != -1) return memo[i][w];

    // a[i - 1] を選ばない場合
    if (func(i - 1, w, a)) return memo[i][w] = 1;

    // a[i - 1] をぶ場合
    if (func(i - 1, w - a[i - 1], a)) return memo[i][w] = 1;

    // どちらも false の場合は false
    return memo[i][w] = 0;
}

int main() {
    // 入力
    int N, W;
    cin >> N >> W;
    vector<int> a(N);
    for (int i = 0; i < N; ++i) cin >> a[i];

    // 再帰的に解く
    memo.assign(N+1, vector<int>(W+1, -1));
    if (func(N, W, a)) cout << "Yes" << endl;
    else cout << "No" << endl;
}
```


```Python
def func (i: int, w: int, a: list):
    ```
    i: 与えられている整数の数
    w: 和となる数。a[i-1] を選ぶ場合には、 w - a[i-1] で部分和を作成できれば OK となる。
    a: 正の整数のリスト  
    ```

    if i == 0:
        if w == 0:
            return True
        else:
            return False
    
    if memo[i][w] != -1:
        return memo[i][w]

    # a[i-1] を選ばない場合
    if func(i-1, w, a):
        return memo[i][w] = 1
    
    # a[i-1] を選ぶ場合
    if func(i-1, w-a[i-1], w):
        return memo[i][w] = 1

    return memo[i][w] = 0


if __main__ == '__name__':
    memo = [-1]*N
    result = func(N, W, a)
```