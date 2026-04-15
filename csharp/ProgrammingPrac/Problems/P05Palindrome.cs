namespace ProgrammingPrac.Problems;

// 問題5: 回文判定 (難易度: ★★☆☆☆)
// 大文字小文字は区別しません。
public static class P05Palindrome
{
    public static bool IsPalindrome(string s)
    {
        var lower = s.ToLowerInvariant();
        return lower == new string(lower.Reverse().ToArray());
    }
}
