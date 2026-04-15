namespace ProgrammingPrac.Problems;

// 問題10: 逆ポーランド記法 (難易度: ★★★★☆)
// 不正な入力の場合は null を返してください。
public static class P10Rpn
{
    public static long? EvalRpn(string expr)
    {
        var tokens = expr.Split(' ',StringSplitOptions.RemoveEmptyEntries);
        var stack = new Stack<long>();
        foreach (string token in tokens) {
            if (long.TryParse(token,out long value)) {
                stack.Push(value);
                continue;
            }
            if (stack.Count<2) {
                return null;
            }
            long b = stack.Pop();
            long a = stack.Pop();
            switch (token) {
                case "+":
                    stack.Push(a+b);
                    break;
                case "-":
                    stack.Push(a-b);
                    break;
                case "*":
                    stack.Push(a*b);
                    break;
                case "/":
                    stack.Push(a/b);
                    break;
                default:
                    return null;
            }
        }
        return stack.Count == 1 ? stack.Pop() : null;
    }
}
