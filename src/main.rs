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
//! umeboshi>> setv x 23
//! Ok
//! umeboshi>> prod u32 $x 2
//! 46
//! umeboshi>> quit
//! Bye!!
//! ```

use std::io::Write;
use std::io;
use std::sync::RwLock;
use std::collections::HashMap;
use termion::{color, style};
use onigiri::tools::chars_to_string;
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

#[derive(PartialEq)]
enum UmeboshiCmd {
    Help,
    Version,
    Echo(Vec<String>),
    Sum(Vec<String>),
    Prod(Vec<String>),
    Getv(Vec<String>),
    Setv(Vec<String>),
}

impl UmeboshiCmd {
    fn run(self) {
        let result = match self {
            UmeboshiCmd::Help => info::help(),
            UmeboshiCmd::Version => format!("{}", VERSION),
            UmeboshiCmd::Echo(e) => e.join(" ").to_string(),
            UmeboshiCmd::Sum(n) => calc::sum(n),
            UmeboshiCmd::Prod(p) => calc::prod(p),
            UmeboshiCmd::Getv(g) => {
                match getv(g[0].to_string()) {
                    Some(v) => v,
                    None => getv("default".to_string()).unwrap(),
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

        let mut v: Vec<&str> = s.trim().split_whitespace().collect();

        let cmd = v[0];
        let params: Vec<String> = v[1..].iter_mut().map(|p| v2v(p.to_string())).collect();

        match cmd {
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
                UmeboshiCmd::Echo(params).run();
                continue;
            },
            "sum" => {
                UmeboshiCmd::Sum(params).run();
                continue;
            },
            "prod" => {
                UmeboshiCmd::Prod(params).run();
                continue;
            },
            "getv" => {
                UmeboshiCmd::Getv(params).run();
                continue;
            },
            "setv" => {
                UmeboshiCmd::Setv(params).run();
                continue;
            },
            _ => {
                break;
            }
        }
    }
}
