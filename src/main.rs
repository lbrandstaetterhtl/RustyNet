use std::io::Write;
use crate::HandleCommand::command::Command;

mod HandleCommand;
mod Help;
mod Operate;
mod Print;

const RED: &str = "\x1B[31m";
const BLUE: &str = "\x1B[34m";
const GREEN: &str = "\x1B[32m";
const BOLD: &str = "\x1B[1m";
const RESET: &str = "\x1B[0m";



fn main() {
    clearscreen::clear().expect("Failed to clearscreen");

    let ping_args = vec!["--s".to_string(), "--t".to_string(), "--c".to_string(), "--h".to_string()];
    let ping: Command = Command::new("ping".to_string(), ping_args, Command::handle_ping, Command::help_ping);

    let null_args = Vec::new();
    let help: Command = Command::new("help".to_string(), null_args, Command::handle_help, Command::help_all);

    let commands: Vec<Command> = vec![ping, help];
    Print::print::header(&commands);
    loop {
        print!("{GREEN}> ");
        std::io::stdout().flush().unwrap();
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();

        HandleCommand::handle::base(&line);
    }
}
