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
    }
}