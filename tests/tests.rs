use rust_practice::problems::*;

// ============================================
// テスト 問題1: あいさつ
// ============================================
#[test]
fn t01_greet_world() {
    assert_eq!(p01_hello::greet("World"), "Hello, World!");
}

#[test]
fn t01_greet_rust() {
    assert_eq!(p01_hello::greet("Rust"), "Hello, Rust!");
}

#[test]
fn t01_greet_empty() {
    assert_eq!(p01_hello::greet(""), "Hello, !");
}

// ============================================
// テスト 問題2: FizzBuzz
// ============================================
#[test]
fn t02_fizzbuzz_3() {
    assert_eq!(p02_fizzbuzz::fizzbuzz(3), "Fizz");
}

#[test]
fn t02_fizzbuzz_5() {
    assert_eq!(p02_fizzbuzz::fizzbuzz(5), "Buzz");
}

#[test]
fn t02_fizzbuzz_15() {
    assert_eq!(p02_fizzbuzz::fizzbuzz(15), "FizzBuzz");
}

#[test]
fn t02_fizzbuzz_7() {
    assert_eq!(p02_fizzbuzz::fizzbuzz(7), "7");
}

// ============================================
// テスト 問題3: 合計値
// ============================================
#[test]
fn t03_sum_basic() {
    assert_eq!(p03_sum::sum(&[1, 2, 3]), 6);
}

#[test]
fn t03_sum_empty() {
    assert_eq!(p03_sum::sum(&[]), 0);
}

#[test]
fn t03_sum_negative() {
    assert_eq!(p03_sum::sum(&[-1, 1, -2, 2]), 0);
}

// ============================================
// テスト 問題4: 文字列の反転
// ============================================
#[test]
fn t04_reverse_hello() {
    assert_eq!(p04_reverse::reverse("hello"), "olleh");
}

#[test]
fn t04_reverse_empty() {
    assert_eq!(p04_reverse::reverse(""), "");
}

#[test]
fn t04_reverse_single() {
    assert_eq!(p04_reverse::reverse("a"), "a");
}

// ============================================
// テスト 問題5: 回文判定
// ============================================
#[test]
fn t05_palindrome_racecar() {
    assert!(p05_palindrome::is_palindrome("racecar"));
}

#[test]
fn t05_palindrome_madam_case() {
    assert!(p05_palindrome::is_palindrome("Madam"));
}

#[test]
fn t05_palindrome_hello() {
    assert!(!p05_palindrome::is_palindrome("hello"));
}

#[test]
fn t05_palindrome_empty() {
    assert!(p05_palindrome::is_palindrome(""));
}

// ============================================
// テスト 問題6: フィボナッチ数列
// ============================================
#[test]
fn t06_fib_0() {
    assert_eq!(p06_fibonacci::fibonacci(0), 0);
}

#[test]
fn t06_fib_1() {
    assert_eq!(p06_fibonacci::fibonacci(1), 1);
}

#[test]
fn t06_fib_10() {
    assert_eq!(p06_fibonacci::fibonacci(10), 55);
}

#[test]
fn t06_fib_20() {
    assert_eq!(p06_fibonacci::fibonacci(20), 6765);
}

// ============================================
// テスト 問題7: 文字の出現回数
// ============================================
#[test]
fn t07_count_hello() {
    let result = p07_count_chars::count_chars("Hello");
    assert_eq!(result[&'h'], 1);
    assert_eq!(result[&'e'], 1);
    assert_eq!(result[&'l'], 2);
    assert_eq!(result[&'o'], 1);
}

#[test]
fn t07_count_spaces_ignored() {
    let result = p07_count_chars::count_chars("A a");
    assert_eq!(result[&'a'], 2);
    assert_eq!(result.len(), 1);
}

#[test]
fn t07_count_empty() {
    let result = p07_count_chars::count_chars("");
    assert!(result.is_empty());
}

// ============================================
// テスト 問題8: 重複の除去
// ============================================
#[test]
fn t08_unique_basic() {
    assert_eq!(p08_unique::unique(&[1, 2, 3, 2, 1]), vec![1, 2, 3]);
}

#[test]
fn t08_unique_all_same() {
    assert_eq!(p08_unique::unique(&[5, 5, 5]), vec![5]);
}

#[test]
fn t08_unique_empty() {
    assert_eq!(p08_unique::unique(&[]), Vec::<i32>::new());
}

// ============================================
// テスト 問題9: 中央値
// ============================================
#[test]
fn t09_median_odd() {
    assert_eq!(p09_median::median(&[3, 1, 2]), 2.0);
}

#[test]
fn t09_median_even() {
    assert_eq!(p09_median::median(&[4, 1, 3, 2]), 2.5);
}

#[test]
fn t09_median_empty() {
    assert_eq!(p09_median::median(&[]), 0.0);
}

#[test]
fn t09_median_single() {
    assert_eq!(p09_median::median(&[42]), 42.0);
}

// ============================================
// テスト 問題10: 逆ポーランド記法
// ============================================
#[test]
fn t10_rpn_add() {
    assert_eq!(p10_rpn::eval_rpn("3 4 +"), Some(7));
}

#[test]
fn t10_rpn_complex() {
    assert_eq!(p10_rpn::eval_rpn("5 1 2 + 4 * + 3 -"), Some(14));
}

#[test]
fn t10_rpn_divide() {
    assert_eq!(p10_rpn::eval_rpn("4 2 /"), Some(2));
}

#[test]
fn t10_rpn_empty() {
    assert_eq!(p10_rpn::eval_rpn(""), None);
}

#[test]
fn t10_rpn_invalid() {
    assert_eq!(p10_rpn::eval_rpn("1 +"), None);
}
