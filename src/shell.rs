use std::io::Write;
use std::io;
use termion::{color, style};


const PRIMARY_PROMPT: &str = "umeboshi>>";

/// Main Loop 
/// quit => Close shell.
pub fn main_loop() {

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

        if s.starts_with("quit") {
            println!("Bye!!");
            break;
        } else {
            let line = s.trim();
            println!("{}", line);
            continue;
        }
    }
}
