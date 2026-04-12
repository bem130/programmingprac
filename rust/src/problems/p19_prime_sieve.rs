// ============================================
// 問題19: エラトステネスの篩 (難易度: ★★★☆☆)
// ============================================
// n 以下の素数を全て求めて Vec で返してください。
//
// アルゴリズム (エラトステネスの篩):
//   1. 2〜n の全ての数を「素数候補」とする
//   2. 最小の候補 p を素数として確定する
//   3. p の倍数 (2p, 3p, ...) を候補から除外する
//   4. 2〜3 を繰り返す
//
// 例:
//   primes(10)  => [2, 3, 5, 7]
//   primes(1)   => []
//   primes(2)   => [2]
//   primes(30)  => [2, 3, 5, 7, 11, 13, 17, 19, 23, 29]
//
// ヒント:
//   - Vec<bool> で「篩い落とされたか」を管理
//   - p*p > n になったら探索終了でOK

pub fn primes(n: usize) -> Vec<usize> {
    // 最小の素数は2
    if n<2 { return vec![]; }
    // 素数候補のboolean配列
    let mut is_prime = vec![true; n + 1];
    // 0,1は必ず除外
    is_prime[0] = false;
    is_prime[1] = false;
    // loop
    // 2から始めて、最小の候補の倍数を除外
    for p in (2..).take_while(|p| p*p<=n) {
        if is_prime[p] {
            for m in (p*p..=n).step_by(p) {
                is_prime[m] = false;
            }
        }
    }
    // 結果の配列を作成
    is_prime.iter().enumerate().filter(|(_,v)| **v).map(|(i, _)| i).collect()
}