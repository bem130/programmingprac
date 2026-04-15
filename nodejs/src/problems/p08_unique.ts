// 問題8: 重複の除去 (難易度: ★★★☆☆)
// 整数の配列から重複を除去し、元の順序を保ったまま返してください。
export function unique(numbers: number[]): number[] {
  const seen = new Set<number>();
  const result: number[] = [];
  for (const num of numbers) {
    if (!seen.has(num)) {
      seen.add(num);
      result.push(num);
    }
  }
  return result;
}
