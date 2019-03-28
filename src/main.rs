// Copyright 2019 Masahiko Hamazawa
//
// Licensed under the MIT license <LICENSE or
//  http://opensource.org/licenses/MIT>.
// This file may not be copied, modified, on distributed except
//  according to thise terms.

use termion::{color, style};
use std::io;
use std::io::Write;
use std::collections::HashMap;
use std::ops::*;

const VERSION: &str = "0.2.0";
const TITLE: &str = "
\t*--------------------*
\t|      umeboshi      |
\t*--------------------*
";
const HELP: &str = r#"
    [Usage]
    quit                    close umeboshi.
    help                    help.
    version                 version information.
    e.g.)
        1 2 + => 3
        12 $x bind => define variable 'x'
        3 4 * $x == => true
        ...
    "#;

#[derive(Debug, PartialEq, PartialOrd)]
struct Object(String);

impl Object {
    fn get_attr(&self) -> String {
        format!("{}", self.0)
    }
}

impl Add for Object {

    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        let _x = self.0.parse::<f64>().unwrap();
        let _y = rhs.0.parse::<f64>().unwrap();
        Object((_x + _y).to_string())
    }
}

impl Sub for Object {

    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        let _x = self.0.parse::<f64>().unwrap();
        let _y = rhs.0.parse::<f64>().unwrap();
        Object((_x - _y).to_string())
    }
}

impl Mul for Object {

    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        let _x = self.0.parse::<f64>().unwrap();
        let _y = rhs.0.parse::<f64>().unwrap();
        Object((_x * _y).to_string())
    }
}

impl Div for Object {

    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        let _x = self.0.parse::<f64>().unwrap();
        let _y = rhs.0.parse::<f64>().unwrap();
        Object((_x / _y).to_string())
    }
}


// Replace variable to value.
fn v2v(var: String) -> String {
    let mut chars: Vec<char> = var.chars().collect();
    if chars[0] == '$' {
        chars.remove(0);
        let v: Vec<String> = chars.iter()
            .map(|c| c.to_string())
            .collect();
        v.concat()
    } else { var }
}

macro_rules! pop2 {
    ( $v:expr ) => ({
        let x = $v.pop().unwrap();
        let y = $v.pop().unwrap();
        (x, y)
    });
}

fn rpn(text: &String, env: &mut HashMap<String, String>) -> String {
    let text2: Vec<&str> = text.split_whitespace().collect();
    let mut stack: Vec<Object> = vec![];

    for t in text2 {
        match t {
            "+" => {
                let (x, y) = pop2!(stack);
                stack.push(x.add(y));
                continue;
            },
            "-" => {
                let (x, y) = pop2!(stack);
                stack.push(y.sub(x));
                continue;
            }, 
            "*" => {
                let (x, y) = pop2!(stack);
                stack.push(x.mul(y));
                continue;
            }, 
            "/" => {
                let (x, y) = pop2!(stack);
                stack.push(y.div(x));
                continue;
            },
            "eq"|"==" => {
                let (x, y) = pop2!(stack);
                stack.push(Object(format!("{}", x == y)));
                continue;
            },
            "not"|"!=" => {
                let (x, y) = pop2!(stack);
                stack.push(Object(format!("{}", x != y)));
                continue;
            },
            "lt"|"<" => {
                let (x, y) = pop2!(stack);
                stack.push(Object(format!("{}", y < x)));
                continue;
            },
            "gt"|">" => {
                let (x, y) = pop2!(stack);
                stack.push(Object(format!("{}", y > x)));
                continue;
            },
            "le"|"<=" => {
                let (x, y) = pop2!(stack);
                stack.push(Object(format!("{}", y <= x)));
                continue;
            },
            "ge"|">=" => {
                let (x, y) = pop2!(stack);
                stack.push(Object(format!("{}", y >= x)));
                continue;
            },
            "bind" => {
                let key = stack.pop().unwrap().0;
                let value = stack.pop().unwrap().0;
                env.insert(key, value);
                continue;
            },
            _ => {
                if t.starts_with("$") {
                    let s = v2v(t.to_string());
                    let value = env.get(&s).unwrap();
                    stack.push(Object(value.to_string()));
                } else {
                    stack.push(Object(t.to_string()))
                }
                continue;
            }
        }
    }
    let stk: Vec<String> = stack.iter().map(|s| s.get_attr()).collect();
    stk.join(" ")
}

// Main Loop
fn main() {
    let mut global_env: HashMap<String, String> = HashMap::new();
    println!("{}{}{}",
             color::Fg(color::Red),
             TITLE,
             style::Reset
             );
    loop {
        let mut s = String::new();
        print!("{}umeboshi>> {}",
               color::Fg(color::Red),
               style::Reset
               );
        io::stdout().flush().expect("Couldn't flush stdout.");
        io::stdin().read_line(&mut s).expect("Failed.");

        if s.starts_with("quit") {
            break;
        } else if s.starts_with("version") {
            println!("{}", VERSION);
            continue;
        } else if s.starts_with("help") {
            println!("{}{}{}",
                    color::Fg(color::Cyan),
                    HELP,
                    style::Reset);
            continue;
        } else {
            println!("{}", rpn(&s, &mut global_env));
            continue;
        }
    }
}
