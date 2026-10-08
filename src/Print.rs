pub mod print {
    use crate::HandleCommand::command::*;
    const BANNER: &str = r#"
 ____               _           _____                _
|  _ \  _   _  ___ | |_  _   _ |_   _|  ___    ___  | |
| |_) || | | |/ __|| __|| | | |  | |   / _ \  / _ \ | |
|  _ < | |_| |\__ \| |_ | |_| |  | |  | (_) || (_) || |
|_| \_\ \__,_||___/ \__| \__, |  |_|   \___/  \___/ |_|
                         |___/
"#;

    const BOLD: &str = "\x1B[1m";
    const GREEN: &str = "\x1B[32m";

    pub fn header() {
        let commands = Command::get_command_list();

        println!("{GREEN}{BOLD}{BANNER}");
        println!("  A command-line tool for network maintenance and analysis.");
        println!(" ");
        println!(" ");
        println!("Commands:");

        let mut counter = 0;
        print!("|");
        for command in commands.values() {
            if command.command != "exit" {
                print!("    {}    ", command.command);

                if counter < commands.values().len() - 1
                {
                    print!("|");
                }
            }
            counter += 1;
        }

        println!(" ");
        println!(" ");
        println!("  Type '<command> --h' to show help for a command.");
        println!("{}", "-".repeat(81));
    }
}