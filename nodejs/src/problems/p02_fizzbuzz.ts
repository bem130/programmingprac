// 問題2: FizzBuzz (難易度: ★☆☆☆☆)
// 3と5の倍数→"FizzBuzz", 3の倍数→"Fizz", 5の倍数→"Buzz", それ以外→数値の文字列
export function fizzbuzz(n: number): string {
  if (n % 15 == 0) return "FizzBuzz";
  if (n % 3 == 0) return "Fizz";
  if (n % 5 == 0) return "Buzz";
  return String(n);
}
