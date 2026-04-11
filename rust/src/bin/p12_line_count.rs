// ============================================
// 問題12: 行数カウント (難易度: ★★☆☆☆)
// ============================================
// 標準入力から複数行を読み取り、行数を出力してください。
//
// 実行例:
//   $ echo -e "aaa\nbbb\nccc" | cargo run --bin p12_line_count
//   3
//
// ヒント:
//   - use std::io::BufRead; すると stdin().lock().lines() が使える
//   - .lines() はイテレータを返すので .count() で行数が取れる

fn main() {
    use std::io::BufRead;
    let count = std::io::stdin().lock().lines().count();
    println!("{}",count);
}
