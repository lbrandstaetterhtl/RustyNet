pub mod help {
    use crate::HandleCommand::command::Command;

    impl Command {
        pub fn help_ping(&self) {
            println!("Usage: {} <target> [OPTIONS]", self.command);
            println!();
            println!("Options:");
            println!("  {:<14} Packet size in bytes", format!("{} <bytes>", self.valid_args[0]));
            println!("  {:<14} Timeout per reply in ms", format!("{} <ms>", self.valid_args[1]));
            println!("  {:<14} Number of pings", format!("{} <count>", self.valid_args[2]));
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
                if command.command != "help" && command.command != "exit" {
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
            println!("  {:<32} {}", a[2], "Show binary representation of the network");
            println!("  {:<32} {}", format!("{} <n>", a[1]), "Split into n equal subnets");
            println!("  {:<32} {}", format!("{} <len>", a[4]), "Split using a new prefix length");
            println!("  {:<32} {}", format!("{} <count count ...>", a[5]), "Split into subnets based on host counts");
            println!("  {:<32} {}", format!("{} <ip>", a[3]), "Check if the network contains a specific IP");
            println!();
            println!("Examples:");
            println!("  {} 10.10.15.0/24 {}", self.command, a[0]);
            println!("  {} 10.10.15.0/24 {} 4", self.command, a[1]);
            println!("  {} 10.10.15.0/24 {} 26", self.command, a[4]);
            println!("  {} 10.10.15.0/24 {} 10.10.15.42", self.command, a[3]);
            println!("  {} 10.10.15.0/24 {} 50 100 200", self.command, a[5]);
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
    }
}