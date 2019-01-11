// Umeboshi is small interactive shell.

use std::sync::RwLock;
use std::collections::HashMap;
use onigiri::tools::chars_to_string;

#[macro_use]
extern crate lazy_static;

mod calc;
mod base;

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
struct UmeboshiCmd {
    cmd: String,
    params: Vec<String>,
}

impl UmeboshiCmd {
    fn new(cmd: String, params: Vec<String>) -> UmeboshiCmd {
        UmeboshiCmd {cmd: cmd, params: params}
    }
    fn run(&mut self) -> Option<String> {
        match self.cmd.as_str() {
            "echo" => Some(self.params.join(" ").to_string()),
            "sum" => Some(calc::sum(&self.params)),
            "prod" => Some(calc::prod(&self.params)),
            "getv" => {
                match getv(self.params[0].to_string()) {
                    Some(v) => Some(v),
                    None => Some(getv("default".to_string()).unwrap()),
                }
            },
            "setv" => {
                setv(self.params[0].to_string(), self.params[1].to_string());
                Some(format!("Ok"))
            },
            "help"|"-h" => Some(base::help()),
            "Version"|"-v" => Some(format!("{}", VERSION)),
            "quit" => None,
            _ => None,
        }
    }
}

macro_rules! ume {
    ( $s:expr ) => ({
        let mut v: Vec<&str> = $s.trim().split_whitespace().collect();
        let cmd = v[0];
        let params: Vec<String> = v[1..].iter_mut()
            .map(|p| v2v(p.to_string()))
            .collect();
        let umeboshicmd = UmeboshiCmd::new(cmd.to_string(), params);
        umeboshicmd
    });
}

fn main() {
    base::title();
    loop {
        let s = base::prompt();
        match ume!(s).run() {
            Some(u) => {
                println!("{}", u);
                continue;
            },
            None => break,
        }
    }
}
