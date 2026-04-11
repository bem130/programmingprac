// ============================================
// 問題7: 文字の出現回数 (難易度: ★★★☆☆)
// ============================================
// 文字列中の各文字の出現回数をHashMapで返してください。
// 大文字小文字は区別しません (すべて小文字としてカウント)。
// スペースは無視してください。
//
// 例:
//   count_chars("Hello") => {'h': 1, 'e': 1, 'l': 2, 'o': 1}
//   count_chars("A a")   => {'a': 2}
//
// ヒント: use std::collections::HashMap;
//         map.entry(key).or_insert(0) を使うと便利

use std::collections::HashMap;

// pub fn count_chars(s: &str) -> HashMap<char, usize> {
//     let mut map = HashMap::new();
//     s.to_lowercase().chars().into_iter().for_each(|c| if c != ' ' {*map.entry(c).or_insert(0)+=1;});
//     map
// }

pub fn count_chars(s: &str) -> HashMap<char, usize> {
    let mut map = HashMap::new();
    for c in s.chars() {
        if c != ' ' {
            *map.entry(c.to_lowercase().next().unwrap()).or_insert(0) += 1;
        }
    }
    map
}