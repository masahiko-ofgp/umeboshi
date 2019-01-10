use termion::{color, style};
use std::io;
use std::io::Write;

// Display Title
pub fn title() {
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

// Disaply prompt
pub fn prompt() -> String {
    let mut s = String::new();
    print!("{}umeboshi>> {}",
           color::Fg(color::Red),
           style::Reset
           );
    io::stdout().flush().expect("Couldn't flush stdout.");
    io::stdin().read_line(&mut s).expect("Failed.");
    s
}

// Display Help
pub fn help() -> String {
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
