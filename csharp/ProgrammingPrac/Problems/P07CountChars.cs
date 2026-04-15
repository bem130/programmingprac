namespace ProgrammingPrac.Problems;

// 問題7: 文字の出現回数 (難易度: ★★★☆☆)
// 大文字小文字は区別しません。スペースは無視してください。
public static class P07CountChars
{
    public static Dictionary<char, int> CountChars(string s)
    {
        var map = new Dictionary<char,int>();
        foreach(char c in s) {
            if (c==' ') {
                continue;
            }
            char key = char.ToLowerInvariant(c);
            if (map.TryGetValue(key,out int count)) {
                map[key] = count+1;
            }
            else {
                map[key] = 1;
            }
        }
        return map;
    }
}
