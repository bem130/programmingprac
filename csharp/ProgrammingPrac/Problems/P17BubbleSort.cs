namespace ProgrammingPrac.Problems;

// 問題17: バブルソート (難易度: ★★☆☆☆)
public static class P17BubbleSort
{
    public static int[] BubbleSort(int[] arr)
    {
        var values = arr.ToArray();
        for (int i=1;i<values.Length;i++) {
            bool swapped = false;
            for (int j=0;j<values.Length-i;j++) {
                if (values[j]>values[j+1]) {
                    int tmp = values[j];
                    values[j] = values[j+1];
                    values[j+1] = tmp;
                    swapped = true;
                }
            }
            if (!swapped) {
                break;
            }
        }
        return values;
    }
}
