# 3.1

```
N個の整数a0,...,aN－1から整数値ai＝vを満たすiを探す以下のコードについて考えます．これは，コード3.2において，breakする処理を省略したものとなっています．もし条件を満たすiが複数通りあった場合には，変数found_idには，そのうちiの値が最大のものが格納されることを確認してください.
```

```Python
a = [a_1, ... , a_N-1]
found_id: int = -1
for i in range(N):
    if a[i] == v:
        found_id = i
```

仮に i = 2, 5, 9 に対して a[i] == v であったとします。このとき、

- まず i = 2 に対して、`found_id = i` の処理によって、found_id = 2 となります
- 次に i = 5 に対して、`found_id = i` の処理によって、found_id = 5 となります
- 最後に i = 9 に対して、`found_id = i` の処理によって、found_id = 9 となります

このように、a[i] == v を満たす i が新たに見つかるたびに、found_id の値がそれに書き換わります。よって最終的な found_id の値は、 a[i] = v を満たす i のうち、最大のものとなります。


# 3.2

```
N個の整数a0,a1,...,aN－1のうち，整数値vが何個含まれるかを求めるO(N)のアルゴリズムを設計してください．
```

次のような実装によって求めることができます。a[i] == v となる i が見つかるたびにインクリメントする変数を用意します。計算量は O(N) です。

　

```cpp
#include <iostream>
#include <vector>
using namespace std;

int main() {
    // 入力を受け取る
    int N, v;
    cin >> N >> v;
    vector<int> a(N);
    for (int i = 0; i < N; ++i) cin >> a[i];

    // 線形探索
    int counter = 0;a
    for (int i = 0; i < N; ++i) {
        if (a[i] == v) {
            ++counter;  // カウントを増やす
        }
    }

    // 結果出力
    cout << counter << endl;
}
```

```Python
if __name__ == '__main__':
    a : list = [a_0, ... , a_N-1]
    count = 0
    for i in a:
        if a[i] == v:
            count += 1
```

# 3.3

```
N(2)個の相異なる整数a0,a1,...,aN－1が与えられます．このうち２番目に小さい値を求めるO(N)のアルゴリズムを設計してください．
```

ポイントは 一番小さな値が新たに出てきた際に、これまで一番小さかった値は、押し出されて二番目に小さな値になる点.次の 2 つの変数を用意します。

- 最小値を表す変数 worst_value
- 2 番目に小さな値を表す変数 second_value

これらを用いて、次のように実装できます。計算量は O(N) です。

```cpp
#include <iostream>
#include <vector>
using namespace std;
const int INF = 20000000;   // 十分大きな値に

int main() {
    // 入力を受け取る
    int N;
    cin >> N;
    vector<int> a(N);
    for (int i = 0; i < N; ++i) cin >> a[i];

    // 線形探索
    int worst_value = INF;
    int second_value = INF;
    for (int i = 0; i < N; ++i) {
        if (a[i] < worst_value) {
            second_value = worst_value;
            worst_value = a[i];
        }
        else if (a[i] < second_value) second_value = a[i];
    }

    // 結果出力
    cout << second_value << endl;
}
```

```Python
if __name__ == '__main__':
    a : list = [a_0, ... , a_N-1]
    count = 0
    for i in a:
        if a[i] == v:
            count += 1
```

　

# 3.4

```
N個の整数a0,a1,...,aN－1が与えられます．この中から２つ選んで差をとります．その差の最大値を求めるO(N)のアルゴリズムを設計してください．
```

*N 個の値の「最大値」と「最小値」を求めます。それらの差が求める値となります。* 計算量は O(N) です。

```cpp
#include <iostream>
#include <vector>
using namespace std;
const int INF = 20000000;   // 十分大きな値に

int main() {
    // 入力を受け取る
    int N;
    cin >> N;
    vector<int> a(N);
    for (int i = 0; i < N; ++i) cin >> a[i];

    // 線形探索
    int max_value = -INF;
    int min_value = INF;
    for (int i = 0; i < N; ++i) {
        if (a[i] < min_value) min_value = a[i];
        if (a[i] > max_value) max_value = a[i];
    }

    // 結果出力
    cout << max_value - min_value << endl;
}
```

```Python
if __name__ == '__main__':
    a : list = [a_0, ... , a_N-1]
    max_value = -123456789
    min_value = 123456789

    for i in a:
        if a[i] > max_value:
            max_value = a[i]
        elif a[i] < min_value:
            min_value = a[i]
    
    result = max_value - min_value
```

# 3.5 (ABC 081 B - Shift Only)

```
N個の正の整数a0,a1,...,aN－1が与えられます．これらに対して「N個の整数がすべて偶数ならば２で割った値に置き換える」という操作を，操作が行えなくなるまで繰り返します．何回の操作を行うことになるかを求めるアルゴリズムを設計して下さい。
```

たとえば、a = (8, 12, 40)​ の場合を考えてみますと

- 8 は 2 で 3 回割り切れます
- 12 は 2 で 2 回割り切れます
- 40 は 2 で 3 回割り切れます

この中で最小回数である 12 (2 回) が全体のボトルネックになることがわかります。つまり操作可能回数は 2 回となります。このことから、次の解法が考えられます。

------

N 個の整数 a[0], a[1], …, a[N-1] に対して、それぞれ「2 で何回割れるか」を求めます。そのうちの最小値が答えになります。

-----


```cpp
#include <iostream>
#include <vector>
using namespace std;

// N が 2 で何回割れるか
int how_many_times(int N) {
    int exp = 0;
    while (N % 2 == 0) N /= 2, ++exp;
    return exp;
}

// 無限大を表す値
const int INF = 1234567890;

int main() {
    int N;
    cin >> N;
    vector<int> A(N);
    for (int i = 0; i < N; ++i) cin >> A[i];

    // 2 で何回割れるかの最小値を求める
    int result = INF;
    for (auto a : A) {
        result = min(result, how_many_times(a));
    }
    cout << result << endl;
}
```


```Python
df how_many_times(N):
    exp = 0
    while (N % 2 == 0):
        N /= 2
        exp += 1
    return exp

result = 123456789

if __name__ == '__main__':
    a : list = [a_0, ... , a_N-1]
    for a_i in a:
        result = min(result, how_many_times(N))
```


# 3.6 (ABC 051 B - Sum of Three Integers)

```
２つの正の整数K,Nが与えられます．0X,Y,ZKを満たす整数(X,Y,Z)の組であってX＋Y＋Z＝Nを満たすものが何通りあるかを求めるO(N2)のアルゴリズムを設計してください．
```

最初に次のような全探索解法が思い浮かぶかもしれません。


```cpp
int counter = 0;
for (int X = 0; X <= K; ++X) {
    for (int Y = 0; Y <= K; ++Y) {
        for (int Z = 0; Z <= K; ++Z) {
            if (X + Y + Z == N) ++counter;
        }
    }
}
```

しかしこのままですと、O(K^3) の計算量となってしまいます。そこで、X + Y + Z = N という関係式に着目します。実は、X, Y の 2 つの値を決めたとき、


Z = N - X - Y

というように、Z の値はただ一つに決まります。この値が 0 <= Z <= K を満たすとは限りませんが、少なくとも Z = N - X - Y でなければなりません。そして、0 <= Z <= K を満たすかどうかを確認します。満たしたならば、個数を表す変数 counter をインクリメントします。

以上をまとめて、次のように実装できます。なお、添字 X のとる範囲を「X <= K」から「X <= min(K, N)」へと改良しています (添字 Y も同様)。計算量は、M = min(K, N) として、O(M^2) となります。

```cpp
#include <iostream>
using namespace std;

int main() {
    int K, N;
    cin >> K >> N;

    int count = 0;
    for (int x = 0; x <= min(K, N); ++x) {
        for (int y = 0; y <= min(K, N); ++y) {
            int z = N - x - y;
            if (z >= 0 && z <= K) {
                ++count;
            }
        }
    }
    cout << count << endl;
}
```

# 3.7 (ABC 045 C - たくさんの数式)


```
各桁の値が１以上９以下の数値のみである整数とみなせるような，長さNの文字列Sが与えられます．この文字列の中で，文字と文字の間のうちのいくつかの場所に「＋」を入れることができます．１つも入れなくてもかまいませんが，「＋」が連続してはいけません．このようにしてできるすべての文字列を数値とみなして，総和を計算するO(2N)のアルゴリズムを設計してください．たとえばS＝"125"のときは，125,1＋25(＝26),12＋5(＝17),1＋2＋5(＝8)の総和をとって176となります．
```

3.5 節で解説した「整数の二進法表現とビット演算」を活用します。長さ N の文字列 S の「隙間」は N-1 箇所あります。そのそれぞれについて

+ 「+」を入れる
+ 「+」を入れない

という 2 つの選択肢があります。よってすべての選択肢は 2^(N-1) 通りとなります。「部分和問題」と同様に、これらの選択肢を 0 以上 2^(N-1) 未満の整数値 (変数 bit) で管理します。

少々実装が複雑ですが、次のように実装できます。計算量は O(N2^N) となります。


```cpp
#include <iostream>
#include <string>
using namespace std;

int main() {
    string S;
    cin >> S;
    int N = S.size();

    // N-1 箇所の隙間に「+」を入れるかどうかをすべて試す
    long long res = 0;
    for (int bit = 0; bit < (1<<(N-1)); ++bit) {
        // tmp：「+」と「+」との間の値を表す変数
        long long tmp = 0;
        for (int i = 0; i < N-1; ++i) {
            tmp *= 10;
            tmp += S[i] - '0';

            // 「+」を挿入するとき、答えに tmp を加算して、tmp を 0 に初期化
            if (bit & (1<<i)) {
                res += tmp;
                tmp = 0;
            }
        }

        // 最後の「+」から残りの部分を答えに加算
        tmp *= 10;
        tmp += S.back() - '0';
        res += tmp;
    }
    cout << res << endl;
}
```


