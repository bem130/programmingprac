namespace ProgrammingPrac.Problems;

// 問題20: 行列の積 (難易度: ★★★☆☆)
public static class P20Matrix
{
    public static long[][] MatMul(long[][] a, long[][] b)
    {
        int rows = a.Length;
        int cols = b[0].Length;
        int common = a[0].Length;
        var result = new long[rows][];
        for (int i=0;i<rows;i++) {
            result[i] = new long[cols];
            for (int j=0;j<cols;j++) {
                for (int k=0;k<common;k++) {
                    result[i][j] += a[i][k]*b[k][j];
                }
            }
        }
        return result;
    }
}
