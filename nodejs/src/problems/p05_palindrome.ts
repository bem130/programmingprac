// 問題5: 回文判定 (難易度: ★★☆☆☆)
// 文字列が回文かどうか判定してください。大文字小文字は区別しません。
export function isPalindrome(s: string): boolean {
  const lower = s.toLowerCase();
  return lower == lower.split("").reverse().join("");
}
