// Copyright 2019 Masahiko Hamazawa
//
// Licensed under the MIT license <LICENSE or
//  http://opensource.org/licenses/MIT>.
// This file may not be copied, modified, on distributed except
//  according to those terms.

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Num(f64),
    Str(String),
    Var(String),
    Bool(bool),
    List(Vec<Token>),
    Plus,
    Minus,
    Times,
    Div,
    Rem,
    Eq,
    Ne,
    Lt,
    Gt,
    Le,
    Ge,
    Define,
    If,
    ListCmd,
    First,
    Rest,
}


impl Token {
    pub fn get_str(&self) -> Option<String> {
        match *self {
            Token::Num(n) => Some(format!("{}", n)),
            Token::Str(ref s) => Some(s.to_string()),
            Token::Bool(b) => Some(format!("{}", b)),
            _ => None
        }
    }
    pub fn add(self, rhs: Self) -> Self {
        match (self, rhs) {
            (Token::Num(x), Token::Num(y)) => Token::Num(x + y),
            _ => panic!("Couldn't add.")
        }
    }
    pub fn sub(self, rhs: Self) -> Self {
        match (self, rhs) {
            (Token::Num(x), Token::Num(y)) => Token::Num(x - y),
            _ => panic!("Couldn't sub.")
        }
    }
    pub fn mul(self, rhs: Self) -> Self {
        match (self, rhs) {
            (Token::Num(x), Token::Num(y)) => Token::Num(x * y),
            _ => panic!("Couldn't mul.")
        }
    }
    pub fn div(self, rhs: Self) -> Self {
        match (self, rhs) {
            (Token::Num(x), Token::Num(y)) => Token::Num(x / y),
            _ => panic!("Couldn't div.")
        }
    }
    pub fn rem(self, rhs: Self) -> Self {
        match (self, rhs) {
            (Token::Num(x), Token::Num(y)) => Token::Num(x % y),
            _ => panic!("Couldn't rem.")
        }
    }
    pub fn eq(self, rhs: Self) -> Self {
        match (self, rhs) {
            (Token::Num(x), Token::Num(y)) => Token::Bool(x == y),
            (Token::Str(x), Token::Str(y)) => Token::Bool(&x[..] == &y[..]),
            (Token::Bool(x), Token::Bool(y)) => Token::Bool(x == y),
            _ => panic!("Couldn't compare.")
        }
    }
    pub fn ne(self, rhs: Self) -> Self {
        match (self, rhs) {
            (Token::Num(x), Token::Num(y)) => Token::Bool(x != y),
            (Token::Str(x), Token::Str(y)) => Token::Bool(&x[..] != &y[..]),
            (Token::Bool(x), Token::Bool(y)) => Token::Bool(x != y),
            _ => panic!("Couldn't compare.")
        }
    }
    pub fn lt(self, rhs: Self) -> Self {
        match (self, rhs) {
            (Token::Num(x), Token::Num(y)) => Token::Bool(x < y),
            (Token::Str(x), Token::Str(y)) => Token::Bool(&x[..] < &y[..]),
            _ => panic!("Couldn't compare.")
        }
    }
    pub fn le(self, rhs: Self) -> Self {
        match (self, rhs) {
            (Token::Num(x), Token::Num(y)) => Token::Bool(x <= y),
            (Token::Str(x), Token::Str(y)) => Token::Bool(&x[..] <= &y[..]),
            _ => panic!("Couldn't compare.")
        }
    }
    // FIXME:
    // :OK:
    //     >> (define x (list 1 2 3))
    //     >> (print $x)
    //     [Num(1.0), Num(2.0), Num(3.0)]
    //     >> (+ (first $x) 5)
    //     6
    //     >> (+ 5 (first (list 1 2 3)))
    //     6
    // :ERROR:
    //     >> (+ (first (list 1 2 3)) 5)
    //     panic!
    pub fn first(&self) -> Option<Self> {
        match self {
            Token::List(l) => Some(l[0].clone()),
            _ => None,
        }
    }
    // FIXME:
    // :Ok:
    //     >> (define x (list 1 2 3))
    //     >> (print $x)
    //     [Num(1.0), Num(2.0), Num(3.0)]
    //     >> (+ (first (rest $x)) 5)
    //     7
    //     >> (+ 5 (first (rest (list 1 2 3))))
    //     7
    // :ERROR:
    //     >> (+ (first (rest (list 1 2 3))) 5)
    //     panic!
    pub fn rest(&self) -> Option<Vec<Self>> {
        match self {
            Token::List(l) => Some(l[1..].to_vec()),
            _ => None,
        }
    }
}
