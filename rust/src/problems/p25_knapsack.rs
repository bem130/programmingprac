// ============================================
// 問題25: 0-1 ナップサック問題 (難易度: ★★★★☆)
// ============================================
// 各アイテムに重さ (weight) と価値 (value) がある。
// 容量 capacity のナップサックに入れられるアイテムの価値の最大値を求めてください。
// 各アイテムは1つしか使えません (0-1 ナップサック)。
//
// 例:
//   knapsack(10, &[(5,10), (4,40), (6,30), (3,50)])
//   => 90  (アイテム2と4を選ぶ: 重さ4+3=7, 価値40+50=90)
//
//   knapsack(0, &[(1,1)]) => 0
//
// アルゴリズム (動的計画法):
//   dp[i][w] = i番目までのアイテムで重さw以下の最大価値
//   - アイテムiを入れない: dp[i-1][w]
//   - アイテムiを入れる:   dp[i-1][w - weight_i] + value_i
//   - dp[i][w] = max(入れない, 入れる)
//
// ヒント:
//   - items は (重さ, 価値) のスライス
//   - 1次元配列で最適化も可能 (逆順にループ)

pub fn knapsack(capacity: usize, items: &[(usize, usize)]) -> usize {
    let n = items.len();
    let mut dp = vec![vec![0usize; capacity+1]; n+1];
    for i in 1..=n {
        let (wi, vi) = items[i-1];
        for w in 0..=capacity {
            dp[i][w] = if wi <= w
                { dp[i-1][w].max(dp[i-1][w-wi]+vi) }
                else { dp[i-1][w] }
        }
    }
    dp[n][capacity]
}
