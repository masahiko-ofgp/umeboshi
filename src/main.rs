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

const TITLE: &str = "
\t*---------------------------*
\t|        umeboshi           |
\t*---------------------------*
";
const VERSION: &str = "0.1.0";
const PRIMARY_PROMPT: &str = "umeboshi>> ";
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
        color::Fg(color::Red), 
        TITLE, 
        style::Reset
    );
    loop {
        let mut s = String::new();
        print!(
            "{}{}{}", 
            color::Fg(color::Red),
            PRIMARY_PROMPT, 
            style::Reset, 
        );
        io::stdout().flush().expect("Couldn't flush stdout");
        io::stdin().read_line(&mut s).expect("Failed.");

        let v: Vec<&str> = s.trim().split_whitespace().collect();
        let (head, tail) = (&v[..1], &v[1..]);

        match head[0] {
            "quit" => {
                println!("Bye!!");
                break;
            },
            "version"|"-v" => {
                println!("{}", VERSION);
                continue;
            },
            "help"|"-h" => {
                println!("{}{}{}", color::Fg(color::Cyan), HELP, style::Reset);
                continue;
            },
            "%" => {
                println!("{}", bind_func(tail));
                continue;
            },
            _ => {
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
fn bind_func<'a>(v: &[&'a str]) -> String {
    let (cmd, params) = (&v[..1], &v[1..]);
    match cmd[0] {
        "echo" => format!("{}", params.join(&" ")),
        "sum" => calc::sum(params.to_vec()),
        "prod" => calc::prod(params.to_vec()),
        _ => format!("Not exist its command."),
    }
}
