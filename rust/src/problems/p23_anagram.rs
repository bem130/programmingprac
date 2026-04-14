// ============================================
// 問題23: アナグラム判定 (難易度: ★★☆☆☆)
// ============================================
// 2つの文字列がアナグラム (同じ文字を並べ替えたもの) かどうか判定してください。
// 大文字小文字は区別しません。スペースは無視します。
//
// 例:
//   is_anagram("listen", "silent") => true
//   is_anagram("Hello", "holle")   => true
//   is_anagram("abc", "abd")       => false
//   is_anagram("Astronomer", "Moon starer") => true
//
// ヒント:
//   - 両方の文字列の文字をソートして比較する
//   - または HashMap で各文字の出現回数を数えて比較する

pub fn is_anagram(a: &str, b: &str) -> bool {
    let mut a: Vec<char> = a.chars().filter(|c| !c.is_whitespace()).flat_map(|c| c.to_lowercase()).collect();
    let mut b: Vec<char> = b.chars().filter(|c| !c.is_whitespace()).flat_map(|c| c.to_lowercase()).collect();
    a.sort();
    b.sort();
    a==b
}
