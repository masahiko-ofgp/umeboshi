// Copyright 2019 Masahiko Hamazawa
//
// Licensed under the MIT license <LICENSE or
//  http://opensource.org/licenses/MIT>.
// This file may not be copied, modified, on distributed except
//  according to those terms.


use super::UmeEnv;
use std::collections::VecDeque;
use std::str::FromStr;


#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Num(f64),
    Str(String),
    Var(String),
    Plus,
    Minus,
    Times,
    Div,
    Eq,
    Ne,
    Lt,
    Gt,
    Le,
    Ge,
    Define,
    Print,
}


impl Token {
    fn get_str(&self) -> Option<String> {
        match *self {
            Token::Num(n) => Some(format!("{}", n)),
            Token::Str(ref s) => Some(s.to_string()),
            _ => None
        }
    }
    fn add(self, rhs: Self) -> Self {
        match (self, rhs) {
            (Token::Num(x), Token::Num(y)) => Token::Num(x + y),
            _ => panic!("Couldn't add.")
        }
    }
    fn sub(self, rhs: Self) -> Self {
        match (self, rhs) {
            (Token::Num(x), Token::Num(y)) => Token::Num(x - y),
            _ => panic!("Couldn't sub.")
        }
    }
    fn mul(self, rhs: Self) -> Self {
        match (self, rhs) {
            (Token::Num(x), Token::Num(y)) => Token::Num(x * y),
            _ => panic!("Couldn't mul.")
        }
    }
    fn div(self, rhs: Self) -> Self {
        match (self, rhs) {
            (Token::Num(x), Token::Num(y)) => Token::Num(x / y),
            _ => panic!("Couldn't div.")
        }
    }
    fn eq(self, rhs: Self) -> Self {
        match (self, rhs) {
            (Token::Num(x), Token::Num(y)) =>
                Token::Str(format!("{:?}", x == y)),
            (Token::Str(x), Token::Str(y)) =>
                Token::Str(format!("{:?}", &x[..] == &y[..])),
            _ => panic!("Couldn't compare.")
        }
    }
    fn ne(self, rhs: Self) -> Self {
        match (self, rhs) {
            (Token::Num(x), Token::Num(y)) =>
                Token::Str(format!("{:?}", x != y)),
            (Token::Str(x), Token::Str(y)) =>
                Token::Str(format!("{:?}", &x[..] != &y[..])),
            _ => panic!("Couldn't compare.")
        }
    }
    fn lt(self, rhs: Self) -> Self {
        match (self, rhs) {
            (Token::Num(x), Token::Num(y)) =>
                Token::Str(format!("{:?}", x < y)),
            (Token::Str(x), Token::Str(y)) =>
                Token::Str(format!("{:?}", &x[..] < &y[..])),
            _ => panic!("Couldn't compare.")
        }
    }
    fn le(self, rhs: Self) -> Self {
        match (self, rhs) {
            (Token::Num(x), Token::Num(y)) =>
                Token::Str(format!("{}", x <= y)),
            (Token::Str(x), Token::Str(y)) =>
                Token::Str(format!("{:?}", &x[..] <= &y[..])),
            _ => panic!("Couldn't compare.")
        }
    }
}

fn to_token<'t>(lexem: &'t str) -> Token {
    match &lexem[..] {
        "+" | "add" => Token::Plus,
        "-" | "sub" => Token::Minus,
        "*" | "mul" => Token::Times,
        "/" | "div" => Token::Div,
        "==" | "eq" => Token::Eq,
        "!=" | "ne" => Token::Ne,
        "<" | "lt" => Token::Lt,
        ">" | "gt" => Token::Gt,
        "<=" | "le" => Token::Le,
        ">=" | "ge" => Token::Ge,
        "define" => Token::Define,
        "print" => Token::Print,
        _ => if lexem.parse::<f64>().is_ok() {
            Token::Num(lexem.parse::<f64>().unwrap())
        } else if lexem.parse::<i64>().is_ok() {
            Token::Num((lexem.parse::<i64>().unwrap()) as f64)
        } else if lexem.starts_with("$") {
            Token::Var(lexem.to_string())
        } else {
            Token::Str(lexem.to_string())
        },
    }
}

fn tokenize<'t>(expr: &'t str) -> VecDeque<Token>{
    let expr = expr.replace("(", " ").replace(")", " ");
    let expr2 = &expr.split_whitespace()
        .collect::<Vec<&str>>()
        .iter()
        .fold(VecDeque::new(), |mut v, l| {
            v.push_front(to_token(l));
            v
        });
    expr2.clone()
}

/// This function change from variable to value.
fn v2v(var: String) -> String {
    let mut chars: Vec<char> = var.chars().collect();
    chars.remove(0);
    chars.iter()
        .fold(String::new(), |mut s, c| {
            s.push_str(&(c.to_string()));
            s
        })
}


pub fn eval<'e>(text: &'e str, env: &mut UmeEnv) -> String {
    let tokens = tokenize(&text);
    let mut stack: Vec<Token> = vec![];

    for tk in tokens.iter() {
        match tk {
            Token::Plus => {
                let x = stack.pop().unwrap();
                let y = stack.pop().unwrap();
                stack.push(x.add(y));
                continue;
            },
            Token::Minus => {
                let x = stack.pop().unwrap();
                let y = stack.pop().unwrap();
                stack.push(x.sub(y));
                continue;
            }, 
            Token::Times => {
                let x = stack.pop().unwrap();
                let y = stack.pop().unwrap();
                stack.push(x.mul(y));
                continue;
            }, 
            Token::Div => {
                let x = stack.pop().unwrap();
                let y = stack.pop().unwrap();
                stack.push(x.div(y));
                continue;
            },
            Token::Eq => {
                let x = stack.pop().unwrap();
                let y = stack.pop().unwrap();
                stack.push(x.eq(y));
                continue;
            },
            Token::Ne => {
                let x = stack.pop().unwrap();
                let y = stack.pop().unwrap();
                stack.push(x.ne(y));
                continue;
            },
            Token::Lt => {
                let x = stack.pop().unwrap();
                let y = stack.pop().unwrap();
                stack.push(y.lt(x));
                continue;
            },
            Token::Gt => {
                let x = stack.pop().unwrap();
                let y = stack.pop().unwrap();
                stack.push(x.lt(y));
                continue;
            },
            Token::Le => {
                let x = stack.pop().unwrap();
                let y = stack.pop().unwrap();
                stack.push(y.le(x));
                continue;
            },
            Token::Ge => {
                let x = stack.pop().unwrap();
                let y = stack.pop().unwrap();
                stack.push(x.le(y));
                continue;
            },
            Token::Define => {
                let key = stack.pop().unwrap().get_str().unwrap();
                let value = stack.pop().unwrap().get_str().unwrap();
                env.insert(key, value);
                continue;
            },
            Token::Print => {
                if stack.len() >= 1 {
                    let mut s = String::new();
                    loop {
                        match stack.pop() {
                            Some(tk) => match tk {
                                Token::Str(st) => s.push_str(
                                    &format!("{} ", &st)
                                    ),
                                Token::Num(n) => s.push_str(
                                    &format!("{}", &n.to_string())
                                    ),
                                _ => break,
                            },
                            None => break,
                        }
                    }
                    stack.push(Token::Str(s));
                    continue;
                } else {
                    continue;
                }
            },
            Token::Var(ref v) => {
                let s = v2v(v.to_string());
                let value = env.get(&s).unwrap();
                stack.push(
                    Token::Str(value.to_string())
                    );
                continue;
            },
            Token::Str(ref st) => {
                if <f64>::from_str(&st).is_ok() {
                    stack.push(
                        Token::Num(st.parse::<f64>().unwrap())
                        );
                } else if <i64>::from_str(&st).is_ok() {
                    stack.push(
                        Token::Num((st.parse::<i64>().unwrap()) as f64)
                        );
                } else {
                    stack.push(
                        Token::Str(st.to_string())
                        );
                }
                continue;
            },
            Token::Num(_) => {
                stack.push(tk.clone());
                continue;
            }
        }
    }
    
    let mut s = String::new();
    loop {
        match stack.pop() {
            Some(tk) => match tk {
                Token::Str(st) => s.push_str(
                    &format!("{} ", &st)
                    ),
                Token::Num(n) => s.push_str(
                    &format!("{}", &n.to_string())
                    ),
                _ => break,
            },
            None => break,
        }
    }
    s.trim_end().to_string()
}
