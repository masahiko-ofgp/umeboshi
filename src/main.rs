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
use termion::{color, style};
use std::thread;
use std::sync::mpsc;
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
    echo [text]             output string.
    sum [type] 1 2 3 ...    output the sum of [type].
        e.g.) sum i32 1 2 3
    prod [type] 1 2 3 ..    output the product of [type].
"#;

fn main() {
    println!("{}{}{}",
             color::Fg(color::Red),
             TITLE,
             style::Reset
             );

    let (sender, receiver) = mpsc::channel();

    loop {
        let mut s = String::new();
        print!("{}{}{}",
               color::Fg(color::Red),
               PRIMARY_PROMPT,
               style::Reset
               );
        io::stdout().flush().expect("Couldn't flush stdout.");
        io::stdin().read_line(&mut s).expect("Failed.");

        match &*s.trim() {
            "quit" => {
                println!("Bye!");
                break;
            },
            "help"|"-h" => {
                println!("{}{}{}",
                        color::Fg(color::Cyan),
                        HELP,
                        style::Reset);
                continue;
            },
            "version"|"-v" => {
                println!("{}", VERSION);
                continue;
            },
            _ => {
                let sender = sender.clone();
                thread::spawn(move || {
                    sender.send(s).unwrap();
                });
            }
        }
        let receive = receiver.recv().unwrap();
        println!("{}", bind_func(receive));
        continue;
    }
}

/// Bind some functions.
fn bind_func(s: String) -> String {
    let mut v: Vec<&str> = s.trim()
        .split_whitespace()
        .collect();

    v.shrink_to_fit();

    let (cmd, params) = (&v[..1], &v[1..]);
    match cmd[0] {
        "echo" => format!("{}", params.join(&" ")),
        "sum" => calc::sum(params.to_vec()),
        "prod" => calc::prod(params.to_vec()),
        _ => format!("Not exist its command."),
    }
}
