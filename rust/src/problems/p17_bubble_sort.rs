// ============================================
// 問題17: バブルソート (難易度: ★★☆☆☆)
// ============================================
// 整数のスライスをバブルソートで昇順に並び替えた Vec を返してください。
// 標準ライブラリの sort() は使わず、自分で実装してください。
//
// アルゴリズム:
//   隣り合う要素を比較し、大小が逆なら交換する。
//   これを全体に対して繰り返す。
//
// 例:
//   bubble_sort(&[5, 3, 1, 4, 2]) => [1, 2, 3, 4, 5]
//   bubble_sort(&[1])              => [1]
//   bubble_sort(&[])               => []
//
// ヒント:
//   - 二重ループ: 外側が回数、内側が隣接比較
//   - swap で交換: vec.swap(i, i+1)

pub fn bubble_sort(arr: &[i32]) -> Vec<i32> {
    let mut vec = arr.to_vec();
    for i in 1..vec.len() {
        let mut swapped = false;
        for j in 0..vec.len()-i {
            if vec[j]>vec[j+1] {
                vec.swap(j, j+1);
                swapped = true;
            }
        }
        if !swapped { break; }
    }
    vec
}