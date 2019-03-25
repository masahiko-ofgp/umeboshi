// Copyright 2019 Masahiko Hamazawa
//
// Licensed under the MIT license <LICENSE or
//  http://opensource.org/licenses/MIT>.
// This file may not be copied, modified, on distributed except
//  according to thise terms.

use termion::{color, style};
use std::io;
use std::io::Write;
use std::sync::RwLock;
use std::collections::HashMap;
use onigiri::tools::chars_to_string;

#[macro_use]
extern crate lazy_static;

mod calc;

const VERSION: &str = "0.1.0";
const TITLE: &str = "
\t*--------------------*
\t|      umeboshi      |
\t*--------------------*
";
const HELP: &str = r#"
    [Usage]
    quit                    close shell.
    help or -h              help.
    version or -v           version information.
    echo [text]             output string.
    sum [type] 1 2 3 ...    output the sum of [type].
        e.g.) sum i32 1 2 3
    prod [type] 1 2 3 ..    output the product of [type].
    "#;

// TODO: I must study it more.
lazy_static! {
    static ref VARS: RwLock<HashMap<String, String>> = {
        let mut vars = HashMap::new();
        vars.insert("none".to_string(), "None".to_string());
        RwLock::new(vars)
    };
}

// Get value from varibable name.
fn getv(key: String) -> Option<String> {
    let vars = VARS.read().unwrap();
    match vars.get(&key) {
        Some(r) => Some(r.to_string()),
        None => None
    }
}

// Set variable.
fn setv(key: String, value: String) {
    let mut vars = VARS.write().unwrap();
    vars.insert(key, value);
}

// Replace variable to value.
fn v2v(s: String) -> String {
    let chars: Vec<char> = s.chars().collect();
    match chars[0] {
        '$' => {
            let var_name = chars_to_string(&(chars[1..]).to_vec());
            getv(var_name).unwrap()
        },
        _ => s
    }
}


// Create UmeboshiCmd from input-String.
/*fn ume<'u>(input: &'u str) -> UmeboshiCmd {
    let cmd = v[0];
    let params: Vec<String> = v[1..].iter_mut()
        .map(|p| v2v(p.to_string()))
        .collect();
    UmeboshiCmd {cmd: cmd.to_string(), params: params}
}*/
#[derive(Debug, PartialEq)]
enum Token {
    Id(String),
    Sum,
    Prod,
    Echo,
    Getv,
    Setv,
}

fn tokenize<'t>(words: Vec<&'t str>) -> Vec<Token> {
    let mut tokens: Vec<Token> = vec![];

    for w in words {
        match w {
            "echo" => tokens.push(Token::Echo),
            "sum" => tokens.push(Token::Sum),
            "prod" => tokens.push(Token::Prod),
            "getv" => tokens.push(Token::Getv),
            "setv" => tokens.push(Token::Setv),
            _ => tokens.push(Token::Id(w.to_string()))
        }
    }
    tokens
}

// Main Loop
fn main() {
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

        let words: Vec<&str> = s.trim()
            .split_whitespace()
            .collect();

        match &words[0] {
            &"quit"|&":q" => break,
            &"version"|&":v" => {
                println!("{}", VERSION);
                continue;
            },
            &"help"|&":h" => {
                println!("{}{}{}",
                         color::Fg(color::Cyan),
                         HELP,
                         style::Reset);
                continue;
            },
            _ => {
                println!("{:?}", tokenize(words));
                continue;
            }
        }
    }
}
