// ============================================
// 問題13: 数値の合計 (難易度: ★★☆☆☆)
// ============================================
// 標準入力から1行に1つずつ整数が入力されます。
// その合計を出力してください。
//
// 実行例:
//   $ echo -e "10\n20\n30" | cargo run --bin p13_sum_lines
//   60
//
// ヒント:
//   - stdin().lock().lines() で各行を取得
//   - 各行を .parse::<i64>() で数値に変換
//   - lines() は Result<String> を返すので .unwrap() が必要

fn main() {
    use std::io::BufRead;
    println!("{}",std::io::stdin().lock().lines().map(|l| { l.unwrap().parse::<i64>().unwrap() }).sum::<i64>());
}
