// ============================================
// 問題14: 簡易grep (難易度: ★★★☆☆)
// ============================================
// コマンドライン引数で検索パターンを受け取り、
// 標準入力から読んだ行のうち、そのパターンを含む行だけを出力してください。
//
// 実行例:
//   $ echo -e "apple\nbanana\napricot" | cargo run --bin p14_grep -- ap
//   apple
//   apricot
//
// ヒント:
//   - std::env::args() でコマンドライン引数を取得
//   - args().nth(1) で最初の引数 (インデックス0はプログラム名)
//   - line.contains(&pattern) で部分一致判定

fn main() {
    let arg = std::env::args().nth(1).unwrap();
    use std::io::BufRead;
    std::io::stdin().lock().lines()
        .filter(|l| { l.as_ref().unwrap().contains(&arg) })
        .for_each(|l| println!("{}", l.unwrap()));
}
