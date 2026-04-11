// ============================================
// 問題8: 重複の除去 (難易度: ★★★☆☆)
// ============================================
// 整数のスライスから重複を除去し、元の順序を保ったまま Vec で返してください。
//
// 例:
//   unique(&[1, 2, 3, 2, 1])     => [1, 2, 3]
//   unique(&[5, 5, 5])           => [5]
//   unique(&[])                  => []
//
// ヒント: HashSet を使って「すでに見た値」を管理しよう
//         use std::collections::HashSet;

pub fn unique(numbers: &[i32]) -> Vec<i32> {
    use std::collections::HashSet;
    let mut set: HashSet<i32> = HashSet::new();
    let mut vec: Vec<i32> = Vec::new();
    for &num in numbers {
        if set.insert(num) {
            vec.push(num);
        }
    }
    vec
}
