// Copyright 2019 Masahiko Hamazawa
//
// Licensed under the MIT license <LICENSE or
//  http://opensource.org/licenses/MIT>.
// This file may not be copied, modified, on distributed except
//  according to those terms.


use super::UmeEnv;
use std::collections::VecDeque;
use std::str::FromStr;
use crate::lexer::Token;


fn to_token<'t>(lexem: &'t str) -> Token {
    match &lexem[..] {
        "+" | "add" => Token::Plus,
        "-" | "sub" => Token::Minus,
        "*" | "mul" => Token::Times,
        "/" | "div" => Token::Div,
        "%" | "rem" => Token::Rem,
        "==" | "eq" => Token::Eq,
        "!=" | "ne" => Token::Ne,
        "<" | "lt" => Token::Lt,
        ">" | "gt" => Token::Gt,
        "<=" | "le" => Token::Le,
        ">=" | "ge" => Token::Ge,
        "true" => Token::Bool(true),
        "false" => Token::Bool(false),
        "if" => Token::If,
        "define" => Token::Define,
        "list" => Token::ListCmd,
        "first" => Token::First,
        "rest" => Token::Rest,
        ";" => Token::Sep,
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


fn to_list(stk: &mut Vec<Token>) -> Vec<Token> {
    let mut list: Vec<Token> = vec![];

    loop {
        match stk.pop() {
            Some(tk) => {
                if tk == Token::Sep {
                    break;
                } else {
                    list.push(tk);
                    continue;
                }
            },
            None => break,
        }
    }

    list
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
            Token::Rem => {
                if let Some((x, y)) = pop2(&mut stack) {
                    stack.push(x.rem(y));
                    continue;
                } else {
                    return "ERROR: `%` and `rem` require 2 params".to_string();
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
            Token::If => {
                match stack.pop() {
                    Some(tk2) => match tk2 {
                        Token::Bool(b) => {
                            if b {
                                let then = stack.pop().unwrap();
                                let _ = stack.pop().unwrap();
                                stack.push(then);
                                continue;
                            } else {
                                let _ = stack.pop().unwrap();
                                continue;
                            }
                        },
                        _ => {
                            return "ERROR: Syntax Error.".to_string();
                        },
                    },
                    None => {
                        return "ERROR: Syntax Error.".to_string();
                    },
                }
            },
            Token::ListCmd => {
                let list = to_list(&mut stack);
                stack.push(Token::List(list.clone()));
                continue;
            },
            Token::List(_) => {
                continue;
            },
            Token::First => {
                let list = stack.pop().unwrap();
                let tk2 = &list.first().unwrap().clone();
                let s = tk2.get_str().unwrap();
                match try_to_f64(&s) {
                    Some(n) => stack.push(Token::Num(n)),
                    None => stack.push(Token::Str(s)),
                }
                continue;
            },
            Token::Rest => {
                let list = stack.pop().unwrap();
                let tokens = &list.rest().unwrap();
                stack.push(Token::List(tokens.to_vec()));
                continue;
            },
            Token::Sep => {
                stack.push(tk.clone());
                continue;
            }
            Token::Var(ref key) => {
                let s = v2v(key.to_string());
                let tk = env.get(&s).unwrap();
                stack.push(tk.clone());
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
            Token::Bool(_) => {
                stack.push(tk.clone());
                continue;
            }
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
                    &format!("{} ", &st.to_string())
                    ),
                Token::Num(n) => s.push_str(
                    &format!("{} ", &n.to_string())
                    ),
                Token::Bool(b) => s.push_str(
                    &format!("{} ", b.to_string())
                    ),
                Token::List(l) => s.push_str(
                    &format!("{:?}", &l)
                    ),
                _ => break,
            },
            None => break,
        }
    }
    s.trim_end().to_string()
}
