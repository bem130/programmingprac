namespace ProgrammingPrac.Problems;

// 問題4: 文字列の反転 (難易度: ★★☆☆☆)
public static class P04Reverse
{
    public static string Reverse(string s)
    {
        return new string(s.Reverse().ToArray());
    }
}
