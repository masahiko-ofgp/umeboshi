//! Umeboshi is small interactive shell.
//! Now, it has 6 commands.
//! ```no_run
//! umeboshi>> help
//! [Usage]
//!     ....
//!     ....
//! umeboshi>> % echo Hello world!
//! Hello world!
//! umeboshi>> % sum u8 12 34
//! 46
//! umeboshi>> % prod f64 100.0 0.75
//! 75
//! umeboshi>> quit
//! Bye!!
//! ```

use std::io::Write;
use std::io;
use termion::{color, style};
mod calc;

const TITLE: &str = r#"
    *--------------*
    |umeboshi shell|
    *--------------*
"#;
const VERSION: &str = "0.1.0";
const PRIMARY_PROMPT: &str = "umeboshi>>";
const HELP: &str = r#"
    [Usage]
    quit                    close shell.
    help or -h              help.
    version or -v           version information.
    % [command]             % is calling function.
    % echo [text]           output string.
    % sum [type] 1 2 3 ...  output the sum of [type].
        e.g.) % sum i32 1 2 3
    % prod [type] 1 2 3 .. output the product of [type].
"#;

/// Main Loop.
fn main() {
    println!(
        "{}{}{}", 
        color::Fg(color::LightRed), 
        TITLE, 
        style::Reset
    );

    loop {
        let mut s = String::new();
        print!(
            "{}{}{} ", 
            color::Fg(color::LightRed),
            PRIMARY_PROMPT, 
            style::Reset, 
        );
        io::stdout().flush().expect("Couldn't flush stdout");
        io::stdin().read_line(&mut s).expect("Failed.");

        let v: Vec<&str> = s.trim().split_whitespace().collect();

        match &v[0] {
            &"quit" => {
                println!("Bye!!");
                break;
            },
            &"version"|&"-v" => {
                println!("{}", VERSION);
                continue;
            },
            &"help"|&"-h" => {
                println!("{}{}{}", color::Fg(color::Cyan), HELP, style::Reset);
                continue;
            },
            &"%" => {
                println!("{}", bind_func(&v));
                continue;
            },
            &_ => {
                println!(
                    "\tPlease input{} {} help{}", 
                    color::Fg(color::LightYellow), 
                    PRIMARY_PROMPT, 
                    style::Reset
                );
                continue;
            }
        }
    }
}

/// Distinction of some functions.
fn bind_func<'a>(v: &Vec<&'a str>) -> String {
    match &v[1] {
        &"echo" => format!("{}", v[2..].join(&" ")),
        &"sum" => format!("{}", calc::sum(v.to_vec())),
        &"prod" => format!("{}", calc::prod(v.to_vec())),
        _ => format!("Not exist its command."),
    }
}
