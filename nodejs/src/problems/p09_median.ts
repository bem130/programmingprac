// 問題9: 中央値 (難易度: ★★★☆☆)
// 整数の配列の中央値を返してください。空の場合は0を返します。
export function median(numbers: number[]): number {
  const values = [...numbers].sort((a, b) => a - b);
  const length = values.length;
  if (length === 0) { return 0; }
  if (length % 2 === 0) { return (values[length / 2 - 1] + values[length / 2]) / 2; }
  return values[Math.floor(length / 2)];
}
