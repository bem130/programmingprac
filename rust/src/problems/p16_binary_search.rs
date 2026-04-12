// ============================================
// 問題16: 二分探索 (難易度: ★★☆☆☆)
// ============================================
// ソート済みの整数スライスから、指定した値のインデックスを返してください。
// 見つからない場合は None を返します。
// 標準ライブラリの binary_search() は使わず、自分で実装してください。
//
// 例:
//   binary_search(&[1, 3, 5, 7, 9], 5) => Some(2)
//   binary_search(&[1, 3, 5, 7, 9], 4) => None
//   binary_search(&[], 1)              => None
//
// ヒント:
//   - left, right の2つの変数で探索範囲を管理
//   - mid = (left + right) / 2 で中央を求める
//   - target と比較して範囲を半分に絞る

// pub fn binary_search(arr: &[i32], target: i32) -> Option<usize> {
//     // init: 全範囲
//     let mut left = 0;
//     let mut right = arr.len();
//     // loop
//     while left<right {
//         let mid = (left+right)/2;
//         if arr[mid]==target {
//             return Some(mid);
//         }
//         else if arr[mid]<target {
//             left = mid+1;
//         }
//         else {
//             right = mid;
//         }
//     }
//     None
// }

pub fn binary_search(arr: &[i32], target: i32) -> Option<usize> {
    // init
    let mut left = 0;
    let mut right = arr.len();
    // loop
    while left<right {
        let mid = left+(right-left)/2;
        use std::cmp::Ordering;
        match arr[mid].cmp(&target) {
            Ordering::Equal => return Some(mid),
            Ordering::Less => left = mid+1,
            Ordering::Greater => right = mid,
        }
    }
    None
}