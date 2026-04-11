use rust_practice::problems::*;

// ============================================
// テスト 問題16: 二分探索
// ============================================
#[test]
fn t16_found() {
    assert_eq!(p16_binary_search::binary_search(&[1, 3, 5, 7, 9], 5), Some(2));
}

#[test]
fn t16_not_found() {
    assert_eq!(p16_binary_search::binary_search(&[1, 3, 5, 7, 9], 4), None);
}

#[test]
fn t16_empty() {
    assert_eq!(p16_binary_search::binary_search(&[], 1), None);
}

#[test]
fn t16_first() {
    assert_eq!(p16_binary_search::binary_search(&[1, 3, 5, 7, 9], 1), Some(0));
}

#[test]
fn t16_last() {
    assert_eq!(p16_binary_search::binary_search(&[1, 3, 5, 7, 9], 9), Some(4));
}

// ============================================
// テスト 問題17: バブルソート
// ============================================
#[test]
fn t17_basic() {
    assert_eq!(p17_bubble_sort::bubble_sort(&[5, 3, 1, 4, 2]), vec![1, 2, 3, 4, 5]);
}

#[test]
fn t17_already_sorted() {
    assert_eq!(p17_bubble_sort::bubble_sort(&[1, 2, 3]), vec![1, 2, 3]);
}

#[test]
fn t17_reverse() {
    assert_eq!(p17_bubble_sort::bubble_sort(&[3, 2, 1]), vec![1, 2, 3]);
}

#[test]
fn t17_empty() {
    assert_eq!(p17_bubble_sort::bubble_sort(&[]), Vec::<i32>::new());
}

#[test]
fn t17_duplicates() {
    assert_eq!(p17_bubble_sort::bubble_sort(&[3, 1, 3, 1]), vec![1, 1, 3, 3]);
}

// ============================================
// テスト 問題18: 最大公約数
// ============================================
#[test]
fn t18_gcd_12_8() {
    assert_eq!(p18_gcd::gcd(12, 8), 4);
}

#[test]
fn t18_gcd_coprime() {
    assert_eq!(p18_gcd::gcd(7, 13), 1);
}

#[test]
fn t18_gcd_100_75() {
    assert_eq!(p18_gcd::gcd(100, 75), 25);
}

#[test]
fn t18_gcd_same() {
    assert_eq!(p18_gcd::gcd(5, 5), 5);
}

// ============================================
// テスト 問題19: エラトステネスの篩
// ============================================
#[test]
fn t19_primes_10() {
    assert_eq!(p19_prime_sieve::primes(10), vec![2, 3, 5, 7]);
}

#[test]
fn t19_primes_1() {
    assert_eq!(p19_prime_sieve::primes(1), Vec::<usize>::new());
}

#[test]
fn t19_primes_2() {
    assert_eq!(p19_prime_sieve::primes(2), vec![2]);
}

#[test]
fn t19_primes_30() {
    assert_eq!(p19_prime_sieve::primes(30), vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29]);
}

// ============================================
// テスト 問題20: 行列の積
// ============================================
#[test]
fn t20_mat_2x2() {
    let a = vec![vec![1, 2], vec![3, 4]];
    let b = vec![vec![5, 6], vec![7, 8]];
    assert_eq!(p20_matrix::mat_mul(&a, &b), vec![vec![19, 22], vec![43, 50]]);
}

#[test]
fn t20_mat_identity() {
    let a = vec![vec![1, 0], vec![0, 1]];
    let b = vec![vec![3, 4], vec![5, 6]];
    assert_eq!(p20_matrix::mat_mul(&a, &b), vec![vec![3, 4], vec![5, 6]]);
}

#[test]
fn t20_mat_rect() {
    let a = vec![vec![1, 2, 3]];         // 1x3
    let b = vec![vec![4], vec![5], vec![6]]; // 3x1
    assert_eq!(p20_matrix::mat_mul(&a, &b), vec![vec![32]]); // 1x1
}

// ============================================
// テスト 問題21: 括弧の対応チェック
// ============================================
#[test]
fn t21_simple() {
    assert!(p21_bracket::is_valid("()"));
}

#[test]
fn t21_multiple() {
    assert!(p21_bracket::is_valid("()[]{}"));
}

#[test]
fn t21_nested() {
    assert!(p21_bracket::is_valid("([{}])"));
}

#[test]
fn t21_mismatch() {
    assert!(!p21_bracket::is_valid("(]"));
}

#[test]
fn t21_unclosed() {
    assert!(!p21_bracket::is_valid("(()"));
}

#[test]
fn t21_with_text() {
    assert!(p21_bracket::is_valid("fn main() { vec![1, 2] }"));
}

// ============================================
// テスト 問題22: ランレングス圧縮・展開
// ============================================
#[test]
fn t22_encode_basic() {
    assert_eq!(p22_run_length::encode("aaabbc"), vec![('a', 3), ('b', 2), ('c', 1)]);
}

#[test]
fn t22_encode_empty() {
    assert_eq!(p22_run_length::encode(""), Vec::<(char, usize)>::new());
}

#[test]
fn t22_encode_single() {
    assert_eq!(p22_run_length::encode("x"), vec![('x', 1)]);
}

#[test]
fn t22_decode_basic() {
    assert_eq!(p22_run_length::decode(&[('a', 3), ('b', 2), ('c', 1)]), "aaabbc");
}

#[test]
fn t22_decode_empty() {
    assert_eq!(p22_run_length::decode(&[]), "");
}

#[test]
fn t22_roundtrip() {
    let original = "aaabbbccccdd";
    assert_eq!(p22_run_length::decode(&p22_run_length::encode(original)), original);
}

// ============================================
// テスト 問題23: アナグラム判定
// ============================================
#[test]
fn t23_anagram_yes() {
    assert!(p23_anagram::is_anagram("listen", "silent"));
}

#[test]
fn t23_anagram_case() {
    assert!(p23_anagram::is_anagram("Hello", "holle"));
}

#[test]
fn t23_anagram_no() {
    assert!(!p23_anagram::is_anagram("abc", "abd"));
}

#[test]
fn t23_anagram_spaces() {
    assert!(p23_anagram::is_anagram("Astronomer", "Moon starer"));
}

// ============================================
// テスト 問題24: 最長共通部分列
// ============================================
#[test]
fn t24_lcs_basic() {
    assert_eq!(p24_longest_common::lcs("abcde", "ace"), 3);
}

#[test]
fn t24_lcs_same() {
    assert_eq!(p24_longest_common::lcs("abc", "abc"), 3);
}

#[test]
fn t24_lcs_none() {
    assert_eq!(p24_longest_common::lcs("abc", "def"), 0);
}

#[test]
fn t24_lcs_complex() {
    assert_eq!(p24_longest_common::lcs("ABCD", "ACBAD"), 3);
}

#[test]
fn t24_lcs_empty() {
    assert_eq!(p24_longest_common::lcs("", "abc"), 0);
}

// ============================================
// テスト 問題25: ナップサック問題
// ============================================
#[test]
fn t25_knapsack_basic() {
    assert_eq!(p25_knapsack::knapsack(10, &[(5, 10), (4, 40), (6, 30), (3, 50)]), 90);
}

#[test]
fn t25_knapsack_zero() {
    assert_eq!(p25_knapsack::knapsack(0, &[(1, 1)]), 0);
}

#[test]
fn t25_knapsack_exact() {
    assert_eq!(p25_knapsack::knapsack(5, &[(5, 100)]), 100);
}

#[test]
fn t25_knapsack_no_items() {
    assert_eq!(p25_knapsack::knapsack(10, &[]), 0);
}
