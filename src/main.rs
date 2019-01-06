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

const VERSION: &str = "0.1.0";

// I try to make global and mutable hashmap. Correct?
// TODO: I must study it more.
lazy_static! {
    static ref VARS: RwLock<HashMap<String, String>> = {
        let mut vars = HashMap::new();
        vars.insert("default".to_string(), "None".to_string());
        RwLock::new(vars)
    };
}

fn main() {
    title();

    loop {
        let mut s = String::new();
        print!("{}umeboshi>> {}",
               color::Fg(color::Red),
               style::Reset
               );
        io::stdout().flush().expect("Couldn't flush stdout.");
        io::stdin().read_line(&mut s).expect("Failed.");

        match &*s.trim() {
            "quit" => {
                println!("Bye!");
                break;
            },
            _ => {
                println!("{}", bind_func(s));
                continue;
            }
        }
    }
}

// TODO: It works. But this function is still under develop.
fn getv<'a>(key: &'a str) -> String {
    let vars = VARS.read().unwrap();
    match vars.get(&key.to_string()) {
        None => getv(&"default".to_string()),
        Some(r) => r.to_string()
    }
}

// TODO: It works. But this function is still under develop.
fn setv<'a>(key: &'a str, value: &'a str) {
    let mut vars = VARS.write().unwrap();
    vars.insert(key.to_string(), value.to_string());
}

/// Bind some functions.
fn bind_func(s: String) -> String {
    let mut v: Vec<&str> = s.trim()
        .split_whitespace()
        .collect();

    v.shrink_to_fit();
    if v.len() > 1 {
        let (cmd, params) = (&v[..1], &v[1..]);
        match cmd[0] {
            "version"|"-v" => format!("{}", VERSION),
            "help"|"-h" => help(),
            "echo" => format!("{}", params.join(&" ")),
            "sum" => calc::sum(params.to_vec()),
            "prod" => calc::prod(params.to_vec()),
            "getv" => getv(params[0]),
            // TODO: I must modified "Ok".to_string" line.
            "setv" => {
                setv(params[0], params[1]);
                "Ok".to_string()
            },
            _ => format!("cmd: {:?} params: {:?}", cmd, params)
        }
    } else {
        "Error: Not exist command or any parameters.".to_string()
    }
}

// Display Title
fn title() {
    let title_text = "
\t*---------------------------*
\t|        umeboshi           |
\t*---------------------------*
    ";
    println!("{}{}{}",
            color::Fg(color::Red),
            title_text,
            style::Reset
            );
}

// Display Help
fn help() -> String {
    let help_text = r#"
    [Usage]
    quit                    close shell.
    help or -h              help.
    version or -v           version information.
    echo [text]             output string.
    sum [type] 1 2 3 ...    output the sum of [type].
        e.g.) sum i32 1 2 3
    prod [type] 1 2 3 ..    output the product of [type].
    "#;
    format!("{}{}{}",
           color::Fg(color::Cyan),
           help_text,
           style::Reset
           )
}
