// 問題6: フィボナッチ数列 (難易度: ★★☆☆☆)
// n番目のフィボナッチ数を返してください (0-indexed)。
export function fibonacci(n: number): number {
  let a = 0;
  let b = 1;
  for (let i = 0; i < n; i++) {
    const tmp = a;
    a = b;
    b = a + tmp;
  }
  return a;
}
