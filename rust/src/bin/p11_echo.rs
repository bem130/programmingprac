// ============================================
// 問題11: エコー (難易度: ★☆☆☆☆)
// ============================================
// 標準入力から1行読み取り、そのまま標準出力に出力してください。
//
// 実行例:
//   $ echo "Hello" | cargo run --bin p11_echo
//   Hello
//
// ヒント:
//   - std::io::stdin().read_line(&mut buf) で1行読める
//   - read_line は末尾に改行 '\n' を含むので .trim() で除去しよう
//   - println! で出力

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    println!("{}", buf.trim());
}
