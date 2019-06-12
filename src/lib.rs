// Copyright 2019 Masahiko Hamazawa
//
// Licensed under the MIT license <LICENSE or
//  http://opensource.org/licenses/MIT>.
// This file may not be copied, modified, on distributed except
//  according to those terms.

use termion::{color, style};
use std::io;
use std::io::Write;
use fnv::FnvHashMap;

pub mod parser;
use parser::eval;

const TITLE: &str = "Umeboshi (>*<)\n";
const HELP: &str = r#"
    [Usage]
    quit or (quit)          close umeboshi.
    help or (help)          help.
    e.g.)
        (+ 1 2) => 3
        (define x 12) => define variable 'x'
        (eq $x (* 3 4)) => true
        ...
    "#;

// REPL Main
pub fn repl() {
    let mut global_env: FnvHashMap<String, String> = FnvHashMap::default();
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

        if s.starts_with("quit")||s.starts_with("(quit)") {
            break;
        } else if s.starts_with("help")||s.starts_with("(help)") {
            println!("{}{}{}",
                    color::Fg(color::Cyan),
                    HELP,
                    style::Reset);
            continue;
        } else {
            println!("{}", eval(&s, &mut global_env));
            continue;
        }
    }
}
