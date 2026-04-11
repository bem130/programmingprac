// ============================================
// 問題10: 逆ポーランド記法 (難易度: ★★★★☆)
// ============================================
// 逆ポーランド記法 (RPN) の式を評価してください。
// トークンはスペースで区切られ、演算子は +, -, *, / の4つです。
// 不正な入力の場合は None を返してください。
//
// 例:
//   eval_rpn("3 4 +")         => Some(7)
//   eval_rpn("5 1 2 + 4 * + 3 -") => Some(14)
//   eval_rpn("4 2 /")         => Some(2)
//   eval_rpn("")               => None
//   eval_rpn("1 +")            => None  (オペランド不足)
//
// ヒント: Vec をスタックとして使おう (push / pop)
//         .parse::<i64>() で文字列を数値に変換できる

#[derive(Debug,PartialEq)]
enum Token {
    Num(i64),
    Operand(Op),
}

#[derive(Debug,PartialEq)]
enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

pub fn eval_rpn(expr: &str) -> Option<i64> {
    // tokenize,parse
    let tokens = expr.split_whitespace().map(|token| {
        match token.parse::<i64>() {
            Ok(n) => Token::Num(n),
            Err(_e) => match token {
                "+" => Token::Operand(Op::Add),
                "-" => Token::Operand(Op::Sub),
                "*" => Token::Operand(Op::Mul),
                "/" => Token::Operand(Op::Div),
                _ => panic!("unknown token"),
            }
        }
    }).collect::<Vec<Token>>();
    // eval
    let mut eval_stack: Vec<i64> = Vec::new();
    for token in tokens {
        match token {
            Token::Num(n) => {
                eval_stack.push(n);
            }
            Token::Operand(op) => {
                match (eval_stack.pop(),eval_stack.pop()) {
                    (Some(b),Some(a)) => {
                        eval_stack.push(match op {
                            Op::Add => a+b,
                            Op::Sub => a-b,
                            Op::Mul => a*b,
                            Op::Div => a/b,
                        });
                    },
                    _ => return None,
                }
            }
        }
    }
    if eval_stack.len()!=1 {None }
    else { eval_stack.pop() }
}
