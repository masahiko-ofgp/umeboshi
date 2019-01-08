//! Umeboshi is small interactive shell.
//! Now, it has 6 commands.
//! ```no_run
//! umeboshi>> help
//! [Usage]
//!     ....
//!     ....
//! umeboshi>> echo Hello world!
//! Hello world!
//! umeboshi>> sum u8 12 34
//! 46
//! umeboshi>> prod f64 100.0 0.75
//! 75
//! umeboshi>> quit
//! Bye!!
//! ```

use std::io::Write;
use std::io;
use std::sync::RwLock;
use std::collections::HashMap;
use termion::{color, style};
#[macro_use]
extern crate lazy_static;
mod calc;
mod info;

const VERSION: &str = "0.1.0";

// TODO: I must study it more.
lazy_static! {
    static ref VARS: RwLock<HashMap<String, String>> = {
        let mut vars = HashMap::new();
        vars.insert("default".to_string(), "None".to_string());
        RwLock::new(vars)
    };
}

// TODO: It works. But this function is still under develop.
fn getv<'a>(key: &'a str) -> Option<String> {
    let vars = VARS.read().unwrap();
    match vars.get(&key.to_string()) {
        Some(r) => Some(r.to_string()),
        None => None
    }
}

// TODO: It works. But this function is still under develop.
fn setv(key: String, value: String) {
    let mut vars = VARS.write().unwrap();
    vars.insert(key, value);
}

#[derive(PartialEq)]
enum UmeboshiCmd<'u> {
    Help,
    Version,
    Echo(Vec<&'u str>),
    Sum(Vec<&'u str>),
    Prod(Vec<&'u str>),
    Getv(Vec<&'u str>),
    Setv(Vec<&'u str>),
}

impl<'u> UmeboshiCmd<'u> {
    fn run(self) {
        let result = match self {
            UmeboshiCmd::Help => info::help(),
            UmeboshiCmd::Version => format!("{}", VERSION),
            UmeboshiCmd::Echo(e) => e.join(" ").to_string(),
            UmeboshiCmd::Sum(n) => calc::sum(n),
            UmeboshiCmd::Prod(p) => calc::prod(p),
            UmeboshiCmd::Getv(g) => {
                match getv(g[0]) {
                    Some(v) => v,
                    None => getv("default").unwrap(),
                }
            },
            UmeboshiCmd::Setv(s) => {
                setv(s[0].to_string(), s[1].to_string());
                format!("Ok")
            },
        };
        println!("{}", result);
    }
}

fn main() {
    info::title();

    loop {
        let mut s = String::new();
        print!("{}umeboshi>> {}",
               color::Fg(color::Red),
               style::Reset
               );
        io::stdout().flush().expect("Couldn't flush stdout.");
        io::stdin().read_line(&mut s).expect("Failed.");

        let v: Vec<&str> = s.trim().split_whitespace().collect();

        let (cmd, params) = (&v[..1], &v[1..]);

        match cmd[0] {
            "quit" => {
                println!("Bye!");
                break;
            },
            "help"|"-h" => {
                UmeboshiCmd::Help.run();
                continue;
            },
            "version"|"-v" => {
                UmeboshiCmd::Version.run();
                continue;
            },
            "echo" => {
                UmeboshiCmd::Echo(params.to_vec()).run();
                continue;
            },
            "sum" => {
                UmeboshiCmd::Sum(params.to_vec()).run();
                continue;
            },
            "prod" => {
                UmeboshiCmd::Prod(params.to_vec()).run();
                continue;
            },
            "getv" => {
                UmeboshiCmd::Getv(params.to_vec()).run();
                continue;
            },
            "setv" => {
                UmeboshiCmd::Setv(params.to_vec()).run();
                continue;
            },
            _ => {
                break;
            }
        }
    }
}
