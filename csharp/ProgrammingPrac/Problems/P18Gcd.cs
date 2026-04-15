namespace ProgrammingPrac.Problems;

// 問題18: 最大公約数 (難易度: ★★☆☆☆)
public static class P18Gcd
{
    public static long Gcd(long a, long b)
    {
        return b==0?a:Gcd(b,a%b);
    }
}
