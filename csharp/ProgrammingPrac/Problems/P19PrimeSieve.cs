namespace ProgrammingPrac.Problems;

// 問題19: エラトステネスの篩 (難易度: ★★★☆☆)
public static class P19PrimeSieve
{
    public static int[] Primes(int n)
    {
        if (n<2) {
            return Array.Empty<int>();
        }
        var isPrime = Enumerable.Repeat(true,n+1).ToArray();
        isPrime[0] = false;
        isPrime[1] = false;
        for (int p=2;p*p<=n;p++) {
            if (isPrime[p]) {
                for (int m=p*p;m<=n;m+=p) {
                    isPrime[m] = false;
                }
            }
        }
        var result = new List<int>();
        for (int i=2;i<=n;i++) {
            if (isPrime[i]) {
                result.Add(i);
            }
        }
        return result.ToArray();
    }
}
