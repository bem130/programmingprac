// ============================================
// 問題22: ランレングス圧縮・展開 (難易度: ★★★☆☆)
// ============================================
// (a) encode: 連続する同じ文字をまとめて (文字, 個数) のペアにする
// (b) decode: ペアのリストから元の文字列を復元する
//
// 例:
//   encode("aaabbc")      => [('a',3), ('b',2), ('c',1)]
//   encode("")            => []
//   decode(&[('a',3), ('b',2), ('c',1)]) => "aaabbc"
//   decode(&[])           => ""
//
// ヒント:
//   encode: 前の文字と比較して、同じならカウント++、違ったら結果に追加
//   decode: 各ペアについて文字を回数分繰り返す (.repeat() が使える)

pub fn encode(s: &str) -> Vec<(char, usize)> {
    let mut s = s.chars();
    let mut res: Vec<(char, usize)> = Vec::new();
    let mut bef = match s.next() {
        Some(v) => v,
        None => return res,
    };
    let mut count = 1;
    for c in s {
        if c==bef {
            count+=1;
        }
        else {
            res.push((bef,count));
            bef = c;
            count = 1;
        }
    }
    res.push((bef,count));
    res
}

pub fn decode(pairs: &[(char, usize)]) -> String {
    let mut s = String::new();
    for &(c,l) in pairs {
        for _ in 0..l { s.push(c); }
    }
    s
}
