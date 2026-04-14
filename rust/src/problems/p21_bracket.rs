// ============================================
// 問題21: 括弧の対応チェック (難易度: ★★★☆☆)
// ============================================
// 文字列に含まれる括弧 (), [], {} が正しく対応しているか判定してください。
// 括弧以外の文字は無視します。
//
// 例:
//   is_valid("()")           => true
//   is_valid("()[]{}")       => true
//   is_valid("(]")           => false
//   is_valid("([{}])")       => true
//   is_valid("(()")          => false
//   is_valid("hello(world)") => true
//
// ヒント:
//   - スタック (Vec) を使う
//   - 開き括弧 → push
//   - 閉じ括弧 → pop して対応する開き括弧か確認
//   - 最後にスタックが空なら OK

#[derive(PartialEq)]
enum Brackets {
    Paren,
    Bracket,
    Brace,
}

pub fn is_valid(s: &str) -> bool {
    let mut stack: Vec<Brackets> = Vec::new();
    for a in s.chars() {
        match a {
            '(' => stack.push(Brackets::Paren),
            '[' => stack.push(Brackets::Bracket),
            '{' => stack.push(Brackets::Brace),
            ')' => { match stack.pop() {
                Some(val) if val == Brackets::Paren => { },
                _ => return false,
            } },
            ']' => { match stack.pop() {
                Some(val) if val == Brackets::Bracket => { },
                _ => return false,
            } },
            '}' => { match stack.pop() {
                Some(val) if val == Brackets::Brace => { },
                _ => return false,
            } },
            _ => { }
        }
    }
    stack.is_empty()
}
