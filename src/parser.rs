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
        _ => if try_to_f64(&lexem).is_some() {
            Token::Num(try_to_f64(&lexem).unwrap())
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

/// Try to convert from str to f64.
fn try_to_f64<'b>(s: &'b str) -> Option<f64> {
    if <f64>::from_str(s).is_ok() {
        Some(s.parse::<f64>().unwrap())
    } else if <i64>::from_str(s).is_ok() {
        Some((s.parse::<i64>().unwrap()) as f64)
    } else {
        None
    }
}


/// This function pop twice.
fn pop2(stk: &mut Vec<Token>) -> Option<(Token, Token)> {
    match (stk.pop(), stk.pop()) {
        (Some(tk1), Some(tk2)) => Some((tk1, tk2)),
        _ => None,
    }
}


pub fn eval<'e>(text: &'e str, env: &mut UmeEnv) -> String {
    let tokens = tokenize(&text);
    let mut stack: Vec<Token> = vec![];

    for tk in tokens.iter() {
        match tk {
            Token::Plus => {
                if let Some((x, y)) = pop2(&mut stack) {
                    stack.push(x.add(y));
                    continue;
                } else {
                    return "ERROR: `+` and `add` require 2 params".to_string();
                }
            },
            Token::Minus => {
                if let Some((x, y)) = pop2(&mut stack) {
                    stack.push(x.sub(y));
                    continue;
                } else {
                    return "ERROR: `-` and `sub` require 2 params".to_string();
                }
            }, 
            Token::Times => {
                if let Some((x, y)) = pop2(&mut stack) {
                    stack.push(x.mul(y));
                    continue;
                } else {
                    return "ERROR: `*` and `mul` require 2 params".to_string();
                }
            }, 
            Token::Div => {
                if let Some((x, y)) = pop2(&mut stack) {
                    stack.push(x.div(y));
                    continue;
                } else {
                    return "ERROR: `/` and `div` require 2 params".to_string();
                }
            },
            Token::Eq => {
                if let Some((x, y)) = pop2(&mut stack) {
                    stack.push(x.eq(y));
                    continue;
                } else {
                    return "ERROR: `==` and `eq` require 2 params".to_string();
                }
            },
            Token::Ne => {
                if let Some((x, y)) = pop2(&mut stack) {
                    stack.push(x.ne(y));
                    continue;
                } else {
                    return "ERROR: `!=` and `ne` require 2 params".to_string();
                }
            },
            Token::Lt => {
                if let Some((x, y)) = pop2(&mut stack) {
                    stack.push(y.lt(x));
                    continue;
                } else {
                    return "ERROR: `<` and `lt` require 2 params".to_string();
                }
            },
            Token::Gt => {
                if let Some((x, y)) = pop2(&mut stack) {
                    stack.push(x.lt(y));
                    continue;
                } else {
                    return "ERROR: `>` and `gt` require 2 params".to_string();
                }
            },
            Token::Le => {
                if let Some((x, y)) = pop2(&mut stack) {
                    stack.push(y.le(x));
                    continue;
                } else {
                    return "ERROR: `<=` and `le` require 2 params".to_string();
                }
            },
            Token::Ge => {
                if let Some((x, y)) = pop2(&mut stack) {
                    stack.push(x.le(y));
                    continue;
                } else {
                    return "ERROR: `>=` and `ge` require 2 params".to_string();
                }
            },
            Token::Define => {
                if let Some((key, value)) = pop2(&mut stack) {
                    env.insert(key.get_str().unwrap(), value);
                    continue;
                } else {
                    return "ERROR: Syntax Error.".to_string();
                }
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
                let tk = env.get(&s).unwrap();
                let value = &tk.get_str().unwrap();
                
                match try_to_f64(&value) {
                    Some(n) => {
                        stack.push(Token::Num(n));
                    },
                    None => {
                        stack.push(Token::Str(value.to_string()));
                    }
                }
                continue;
            },
            Token::Str(ref st) => {
                match try_to_f64(&st) {
                    Some(n) => {
                        stack.push(Token::Num(n));
                    },
                    None => {
                        stack.push(Token::Str(st.to_string()));
                    }
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