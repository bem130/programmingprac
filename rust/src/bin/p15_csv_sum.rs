// ============================================
// 問題15: CSV集計 (難易度: ★★★☆☆)
// ============================================
// 標準入力からCSV形式 (名前,数値) のデータを読み取り、
// 名前ごとの合計を出力してください。出力順は名前のアルファベット順です。
//
// 実行例:
//   $ echo -e "alice,10\nbob,20\nalice,30" | cargo run --bin p15_csv_sum
//   alice,40
//   bob,20
//
// ヒント:
//   - line.split(',') でカンマ区切りに分割
//   - HashMap で名前ごとの合計を管理
//   - BTreeMap を使うと自動的にキーがソートされる
//     use std::collections::BTreeMap;

fn main() {
    use std::collections::BTreeMap;
    use std::io::BufRead;
    let mut map: BTreeMap<String,i64> = BTreeMap::new();
    std::io::stdin().lock().lines()
        .for_each(|l| {
            let mut parts = l.as_ref().unwrap().splitn(2,',');
            let name = parts.next().unwrap();
            let value = parts.next().unwrap().parse::<i64>().unwrap();
            *map.entry(name.to_string()).or_insert(0) += value;
        });
    for (name,total) in map {
        println!("{},{}",name,total);
    }
}
