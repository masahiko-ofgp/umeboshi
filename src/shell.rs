use std::io::Write;
use std::io;
use termion::{color, style};

const VERSION: &str = "0.1.0";
const PRIMARY_PROMPT: &str = "umeboshi>>";
const HELP: &str = r#"
    [Usage]
    quit:                close shell.
    % help or % -h:      help.
    % version or % -v:   version information.
    % echo [text]:       output string.
"#;

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

        let v: Vec<&str> = s.trim().split_whitespace().collect();

        if &v[0] == &"quit" {
            println!("Bye!!");
            break;
        } else if &v[0] == &"%" {
            bind_func(&v);
            continue;
        } else {
            println!(
                "\t{}Please input {} % help{}", 
                color::Fg(color::LightYellow), 
                PRIMARY_PROMPT, 
                style::Reset
            );
            continue;
        }
    }
}

fn bind_func<'b>(v: &Vec<&'b str>) {
    match &v[1] {
        &"echo" => println!("{:?}", &v[2..].join(&" ")),
        &"help"|&"-h" => println!("{}{}{}", color::Fg(color::Cyan), HELP, style::Reset),
        &"version"|&"-v" => println!("{}", VERSION),
        _ => println!("Not exist its command."),
    }
}


