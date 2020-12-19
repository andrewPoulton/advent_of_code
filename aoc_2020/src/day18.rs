use crate::utils::file2vec;
use crate::day18_assist;

pub fn day18(filename: &String){
    let contents = file2vec::<String>(filename);
    let contents:Vec<String> =  contents.iter().map(|x| x.to_owned().unwrap()).collect();
    // println!("{}", contents[0]);
    // println!("{}", evaluate("4 + ((3 * 2) + 1 * 9)"));
    let part1_ans = contents.iter().map(|exp| evaluate(&exp[..])).sum::<i64>();
    println!("part 1 ans: {}", part1_ans);
    let part2_ans = day18_assist::calculate();
    println!("part 2 ans: {}", part2_ans);
}


fn evaluate(equation: &str)->i64{
    let mut stack = vec![State {val: 0, op: None }];
    for ch in equation.chars(){
        // println!("{}:  {:?}",ch, stack);
        match ch {
            '('=> {
                stack.push(State {val: 0, op: None });
            }
            x if x.is_numeric() => {
                if let Some(val) = stack.last_mut() {
                    match val.op {
                        Some(Op::add) => {
                            val.val += x.to_string().parse::<i64>().unwrap_or(0);
                        },
                        Some(Op::mul) => {
                            val.val *= x.to_string().parse::<i64>().unwrap_or(1);
                        },
                        None => {
                            val.val  = x.to_string().parse::<i64>().unwrap_or(0);
                        }
                    } 
                }
            },
            '+' => {
                if let Some(val) =  stack.last_mut(){
                    val.op = Some(Op::add);
                }
            },
            '*' => {
                if let Some(val) = stack.last_mut(){
                    val.op = Some(Op::mul);
                }
            },
            ')' => {
                let inner_expression = stack.pop().unwrap();
                if let Some(val) = stack.last_mut() {
                    match val.op {
                        Some(Op::add) => {
                            val.val += inner_expression.val;
                        },
                        Some(Op::mul) => {
                            val.val *= inner_expression.val;
                        },
                        None => {
                            val.val = inner_expression.val;
                        }
                    } 
                }
            },
            _ => ()
        }
    }
    stack.pop().unwrap().val
}

#[derive(Debug)]
struct State{
    val: i64,
    op: Option<Op>
}

#[derive(Debug)]
enum Op{
    add,
    mul
}

//part 2 steal lexer and use Pratt parsing
//could switch + and * and overload to reverse meaning, and then use eval I think (or hardcode transformed expressions somewhere)

// use std::fmt;

// enum S {
//     Atom(char),
//     Cons(char, Vec<S>),
// }

// impl fmt::Display for S {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             S::Atom(i) => write!(f, "{}", i),
//             S::Cons(head, rest) => {
//                 write!(f, "({}", head)?;
//                 for s in rest {
//                     write!(f, " {}", s)?
//                 }
//                 write!(f, ")")
//             }
//         }
//     }
// }

// fn expr(input: &str) -> S {
//     let mut lexer = Lexer::new(input);
//     expr_bp(&mut lexer, 0) 
// }

// fn expr_bp(lexer: &mut Lexer, min_bp: u8) -> S { 
//     let mut lhs = match lexer.next() {
//         Token::Atom(it) => S::Atom(it),
//         t => panic!("bad token: {:?}", t),
//     };

//     loop {
//         let op = match lexer.peek() {
//             Token::Eof => break,
//             Token::Op(op) => op,
//             t => panic!("bad token: {:?}", t),
//         };

//         let (l_bp, r_bp) = infix_binding_power(op);
//         if l_bp < min_bp { 
//             break;
//         }

//         lexer.next(); 
//         let rhs = expr_bp(lexer, r_bp);

//         lhs = S::Cons(op, vec![lhs, rhs]); 
//     }

//     lhs
// }

// fn infix_binding_power(op: char) -> (u8, u8) {
//     match op {
//         '+' | '-' => (1, 2),
//         '*' | '/' => (3, 4),
//         _ => panic!("bad op: {:?}"),
//     }
// }


