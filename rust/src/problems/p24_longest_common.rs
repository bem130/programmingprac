// ============================================
// 問題24: 最長共通部分列 (LCS) (難易度: ★★★★☆)
// ============================================
// 2つの文字列の最長共通部分列 (Longest Common Subsequence) の長さを返してください。
// 部分列は連続していなくてもOK (部分"文字列"とは違う)。
//
// 例:
//   lcs("abcde", "ace")   => 3   ("ace")
//   lcs("abc", "abc")     => 3
//   lcs("abc", "def")     => 0
//   lcs("ABCD", "ACBAD")  => 3   ("ABD" や "ACD" など)
//
// アルゴリズム (動的計画法):
//   dp[i][j] = s1[0..i] と s2[0..j] の LCS の長さ
//   - s1[i-1] == s2[j-1] なら dp[i][j] = dp[i-1][j-1] + 1
//   - そうでなければ dp[i][j] = max(dp[i-1][j], dp[i][j-1])
//
// ヒント:
//   - vec![vec![0; n+1]; m+1] で2次元配列を作れる
//   - .chars().nth(i) で i 番目の文字を取得

pub fn lcs(s1: &str, s2: &str) -> usize {
    let a: Vec<char> = s1.chars().collect();
    let b: Vec<char> = s2.chars().collect();
    let n = a.len();
    let m = b.len();
    let mut dp = vec![vec![0usize; m+1]; n+1];
    for i in 1..=n {
        for j in 1..=m {
            if a[i-1]==b[j-1] {
                dp[i][j] = dp[i-1][j-1]+1;
            }
            else {
                dp[i][j] = dp[i-1][j].max(dp[i][j-1]);
            }
        }
    }
    dp[n][m]
}
