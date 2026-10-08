pub mod print {
    use crate::HandleCommand::command::*;
    use crate::Operate::operate::*;
    use std::net::{IpAddr, Ipv4Addr};
    use crate::Operate::operate;
    use crate::Print::print;

    const BANNER: &str = r#"
 ____               _             _   _        _
|  _ \  _   _  ___ | |_  _   _   | \ | |  ___ | |_
| |_) || | | |/ __|| __|| | | |  |  \| | / _ \| __|
|  _ < | |_| |\__ \| |_ | |_| |  | |\  ||  __/| |_
|_| \_\ \__,_||___/ \__| \__, |  |_| \_| \___| \__|
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

        print!("|");
        for command in commands.values() {
            if command.command != "exit" && command.command != "clear" {
                print!("    {}    ", command.command);
                print!("|");
            }
        }

        println!(" ");
        println!(" ");
        println!("  Type '<command> --h' to show help for a command.");
        println!("  Type 'clear' to clear the screen.");
        println!("  Type 'exit' to exit the application.");
        println!("{}", "-".repeat(81));
    }

    pub fn network_info(info: &NetInfo) {
        println!();
        println!("  {:<16} {}/{}", "Network:", info.network_addr, info.prefix);
        println!("  {:<16} {}", "Broadcast:", info.broadcast_addr);
        println!("  {:<16} {}", "Subnet mask:", Ipv4Addr::from(info.mask));
        println!("  {:<16} {}", "Wildcard:", Ipv4Addr::from(!info.mask));
        println!("  {:<16} {}", "First host:", info.first_host);
        println!("  {:<16} {}", "Last host:", info.last_host);
        println!("  {:<16} {}", "Usable hosts:", info.host_count);
        println!();
    }

    pub fn network_info_list(nets: Vec<String>) {
        for net in nets {
            let info = get_network_info(&net);

            if info.is_err() {
                println!("{}", info.err().unwrap().to_string());
                return;
            }

            network_info(&info.ok().unwrap());
        }
    }
}