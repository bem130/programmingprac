namespace ProgrammingPrac.Problems;

// 問題9: 中央値 (難易度: ★★★☆☆)
public static class P09Median
{
    public static double Median(int[] numbers)
    {
        var values = numbers.ToArray();
        Array.Sort(values);
        int length = values.Length;
        if (length==0) {
            return 0.0;
        }
        if (length%2==0) {
            return (values[length/2-1]+values[length/2])/2.0;
        }
        return values[length/2];
    }
}
