pub mod help {
    use crate::HandleCommand::command::Command;

    impl Command {
        pub fn help_ping(&self) {
            println!("Usage: {} <target> [OPTIONS]", self.command);
            println!();
            println!("Options:");
            println!("  {:<14} Packet size in bytes (default: 64 bytes)", format!("{} <bytes>", self.valid_args[0]));
            println!("  {:<14} Timeout per reply in ms (default: 500ms)", format!("{} <ms>", self.valid_args[1]));
            println!("  {:<14} Number of pings (default: 4)", format!("{} <count>", self.valid_args[2]));
            println!();
            println!("Example:");
            println!("  {} google.com {} 4 {} 64 {} 1000",
                     self.command, self.valid_args[2], self.valid_args[0], self.valid_args[1]);
        }

        pub fn help_all(&self)
        {
            let commands = Command::get_command_list();

            println!("---------------------------------------------------------------------------------");
            for command in commands.values() {
                if command.command != "help" && command.command != "exit" && command.command != "clear" {
                    let help = command.help_fn;
                    help(&command);
                    println!("---------------------------------------------------------------------------------");
                }
            }
        }

        pub fn help_netcalc(&self) {
            let a = &self.valid_args;

            println!();
            println!("Usage: {} <network/prefix> [option]", self.command);
            println!();
            println!("Arguments:");
            println!("  {:<32} {}", "network/prefix", "Network in CIDR notation (e.g. 10.10.15.0/24)");
            println!();
            println!("Options:");
            println!("  {:<32} {}", a[0], "Show all info for the given network");
            println!("  {:<32} {}", format!("{} <n>", a[1]), "Split into n equal subnets");
            println!("  {:<32} {}", format!("{} <len>", a[3]), "Split using a new prefix length");
            println!("  {:<32} {}", format!("{} <ip>", a[2]), "Check if the network contains a specific IP");
            println!();
            println!("Examples:");
            println!("  {} 10.10.15.0/24 {}", self.command, a[0]);
            println!("  {} 10.10.15.0/24 {} 4", self.command, a[1]);
            println!("  {} 10.10.15.0/24 {} 10.10.15.42", self.command, a[3]);
            println!();
        }

        pub fn help_trace(&self) {
            let a = &self.valid_args;   // 0 hops, 1 timeout, 2 resolve

            println!();
            println!("Usage: {} <host> [options]", self.command);
            println!();
            println!("Arguments:");
            println!("  {:<25} {}", "host", "Target hostname or IP address");
            println!();
            println!("Options:");
            println!("  {:<25} {}", format!("{} <n>", a[0]), "Maximum number of hops (default: 30)");
            println!("  {:<25} {}", format!("{} <ms>", a[1]), "Timeout per hop in ms (default: 2000)");
            println!("  {:<25} {}", a[2], "Resolve hostnames for each hop");
            println!();
            println!("Examples:");
            println!("  {} 8.8.8.8", self.command);
            println!("  {} google.com {} 20 {}", self.command, a[0], a[2]);
            println!();
        }

        pub fn help_portscan(&self) {
            let a = &self.valid_args;

            println!();
            println!("Usage: {} <host> [options]", self.command);
            println!();
            println!("Arguments:");
            println!("  {:<25} {}", "host", "Target hostname or IP address");
            println!();
            println!("Options:");
            println!("  {:<25} {}", format!("{} <n,n,...>", a[0]), "Scan specific ports (comma-separated)");
            println!("  {:<25} {}", format!("{} <start-end>", a[1]), "Scan a range of ports");
            println!("  {:<25} {}", a[2], "Scan common ports (22, 80, 443, 3389, ...)");
            println!("  {:<25} {}", format!("{} <ms>", a[3]), "Timeout per port in ms (default: 500)");
            println!();
            println!("Examples:");
            println!("  {} 192.168.0.1 {}", self.command, a[2]);
            println!("  {} 192.168.0.1 {} 22,80,443", self.command, a[0]);
            println!("  {} 192.168.0.1 {} 1-1024 {} 100", self.command, a[1], a[3]);
            println!();
        }
    }
}