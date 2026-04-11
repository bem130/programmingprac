// ============================================
// 問題5: 回文判定 (難易度: ★★☆☆☆)
// ============================================
// 文字列が回文(前から読んでも後ろから読んでも同じ)かどうか判定してください。
// 大文字小文字は区別しません。
//
// 例:
//   is_palindrome("racecar") => true
//   is_palindrome("Madam")   => true
//   is_palindrome("hello")   => false
//   is_palindrome("")         => true
//
// ヒント: .to_lowercase() で小文字にしてから比較しよう

pub fn is_palindrome(s: &str) -> bool {
    let lower = s.to_lowercase();
    lower == lower.chars().rev().collect::<String>()
}
