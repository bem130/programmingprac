namespace ProgrammingPrac.Problems;

// 問題6: フィボナッチ数列 (難易度: ★★☆☆☆)
public static class P06Fibonacci
{
    public static long Fibonacci(int n)
    {
        long a = 0;
        long b = 1;
        for (int i=0;i<n;i++) {
            long tmp = a;
            a = b;
            b = a+tmp;
        }
        return a;
    }
}
