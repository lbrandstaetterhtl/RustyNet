use std::io::Write;

mod HandleCommand;
mod Help;
mod Operate;


const RED: &str = "\x1B[31m";
const BLUE: &str = "\x1B[34m";
const GREEN: &str = "\x1B[32m";
const BOLD: &str = "\x1B[1m";
const RESET: &str = "\x1B[0m";



fn main() {
    clearscreen::clear().expect("Failed to clearscreen");
    loop {
        print!("{GREEN}>");
        std::io::stdout().flush().unwrap();
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();

        HandleCommand::handle::base(&line);
    }
}
