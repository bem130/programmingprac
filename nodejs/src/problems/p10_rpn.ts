// 問題10: 逆ポーランド記法 (難易度: ★★★★☆)
// 逆ポーランド記法 (RPN) の式を評価してください。
// 不正な入力の場合は null を返してください。
export function evalRpn(expr: string): number | null {
  const tokens = expr.trim() === "" ? [] : expr.trim().split(/\s+/);
  const stack: number[] = [];
  for (const token of tokens) {
    const value = Number(token);
    if (!Number.isNaN(value)) {
      stack.push(value);
      continue;
    }
    if (stack.length < 2) {
      return null;
    }
    const b = stack.pop()!;
    const a = stack.pop()!;
    switch (token) {
      case "+":
        stack.push(a + b);
        break;
      case "-":
        stack.push(a - b);
        break;
      case "*":
        stack.push(a * b);
        break;
      case "/":
        stack.push(a / b);
        break;
      default:
        return null;
    }
  }
  return stack.length === 1 ? stack[0] : null;
}
