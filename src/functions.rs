use std::io::Write;
use std::io;
use termion::{color, style};
use onigiri::tools::{Vvc, chars_to_string};

/// Main Loop 
/// quit => Close shell.
/// % => Symbol for calling function. 
pub fn main_loop() {
    let mut cmd_count: usize = 1;

    loop {
        let mut s = String::new();
        print!(
            "{}umeboshi{}:{}[IN:{}]{} ", 
            color::Fg(color::LightRed), 
            style::Reset, 
            color::Fg(color::LightRed),
            &cmd_count,
            style::Reset,
        );
        io::stdout().flush().expect("Couldn't flush stdout");
        io::stdin().read_line(&mut s).expect("Failed.");

        if s.starts_with("quit") {
            println!("Bye!!");
            break;
        } else if s.starts_with("% ") {
            let vvc = to_vvc(&s);
            let (fname, params) = div_into(vvc);
            println!(
                "{}[OUT:{} FUNC:{}]{} {:?}", 
                color::Fg(color::LightYellow),
                &cmd_count,
                fname,
                style::Reset,
                params,
            );
            cmd_count += 1;
            continue;
        } else {
            let lit = &s.trim_end();
            println!(
                "{}[OUT:{}]{} {:?}", 
                color::Fg(color::Green),
                &cmd_count,
                style::Reset, 
                &lit,
            );
            cmd_count += 1;
            continue;
        }
    }
}

/// Input text's end is "\n".
/// Therefore, this function remove it.
pub fn to_vvc<'l>(line: &'l str) -> Vvc {
    let line = line.trim_end().to_string();
    let new_vvc = Vvc::new(&line, ' ');
    new_vvc
}

/// This function divide line into func_name and params.
/// e.g.) % add 14 -23 => ("add", ["14", "-23"])
pub fn div_into(line: Vvc) -> (String, Vec<String>) {
    let func_name = chars_to_string(&line.attr[1]);

    let mut params: Vec<String> = vec![];
    
    for l in &line.attr[2..] {
        params.push(chars_to_string(&l));
    }

    (func_name, params)
}
