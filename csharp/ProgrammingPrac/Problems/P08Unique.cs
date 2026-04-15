namespace ProgrammingPrac.Problems;

// 問題8: 重複の除去 (難易度: ★★★☆☆)
public static class P08Unique
{
    public static int[] Unique(int[] numbers)
    {
        var set = new HashSet<int>();
        var list = new List<int>();
        foreach (int num in numbers) {
            if (set.Add(num)) {
                list.Add(num);
            }
        }
        return list.ToArray();
    }
}
