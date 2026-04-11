// ============================================
// 問題6: フィボナッチ数列 (難易度: ★★☆☆☆)
// ============================================
// n番目のフィボナッチ数を返してください (0-indexed)。
//   fib(0) = 0, fib(1) = 1, fib(n) = fib(n-1) + fib(n-2)
//
// 例:
//   fibonacci(0)  => 0
//   fibonacci(1)  => 1
//   fibonacci(10) => 55
//
// ヒント: ループで前の2つの値を保持しながら計算しよう
//         (再帰でも書けるが、ループの方が効率的)

// pub fn fibonacci(n: u32) -> u64 {
//     match n {
//         0 => 0,
//         1 => 1,
//         n => fibonacci(n-1)+fibonacci(n-2),
//     }
// }

pub fn fibonacci(n: u32) -> u64 {
    let mut a = 0;
    let mut b = 1;
    for _ in 0..n {
        let tmp = a;
        a = b;
        b = a + tmp;
    }
    a
}