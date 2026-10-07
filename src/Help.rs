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
            let ping_args = vec!["--s".to_string(), "--t".to_string(), "--c".to_string(), "--h".to_string()];
            let ping: Command = Command::new("ping".to_string(), ping_args, Command::handle_ping, Command::help_ping);

            let commands: Vec<Command> = vec![ping];
            println!("---------------------------------------------------------------------------------");
            for command in commands {
                let help = command.help_fn;
                help(&command);
                println!("---------------------------------------------------------------------------------");
            }
        }
    }
}