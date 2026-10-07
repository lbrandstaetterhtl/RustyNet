pub mod command {
    use crate::Help::help;
    use crate::Operate::operate;

    pub struct Command {
        pub valid_args: Vec<String>,
        pub command: String,
        pub handle_fn: fn(&Command, &Input),
        pub help_fn: fn(&Command)
    }

    impl Command {

        pub fn handle_ping(&self, input: &Input) {
            let mut count = None;
            let mut size = None;
            let mut timeout = None;

            let mut i = 0;
            while i < input.args.len() {
                if input.args[i] == self.valid_args[0] {
                    size = Some(input.args[i + 1].trim().parse::<u32>().unwrap());
                    i += 1;
                }
                else if input.args[i] == self.valid_args[1] {
                    timeout = Some(input.args[i + 1].trim().parse::<u32>().unwrap());
                    i += 1;
                }
                else if input.args[i] == self.valid_args[2] {
                    count = Some(input.args[i + 1].trim().parse::<u32>().unwrap());
                    i += 1;
                }
                else {
                    println!("{} is not a valid command", input.args[i]);
                }
                i += 1;
            }

            operate::operate_ping(count, size, timeout, &input.target);
        }

        pub const fn new(command: String, valid_args: Vec<String>, handle_fn: fn(&Command, &Input), help_fn: fn(&Command)) -> Command {
           return Command {
               valid_args,
               command,
               handle_fn,
               help_fn
           };
        }

        pub fn get_command(input: &Input) -> Command {
            let ping_args = vec!["--s".to_string(), "--t".to_string(), "--c".to_string(), "--h".to_string()];
            let ping: Command = Command::new("ping".to_string(), ping_args, Command::handle_ping, Command::help_ping);

            if input.command == ping.command {
                return ping;
            }
            return ping;
        }
    }

    pub struct Input {
        pub args: Vec<String>,
        pub command: String,
        pub target: String
    }

    impl Input {
        pub fn new(args: Vec<String>, command: String, target: String) -> Input {
            return Input {
                args,
                command,
                target
            }
        }
    }
}

pub mod handle {
    use crate::HandleCommand::command::Command;
    use crate::HandleCommand::command::Input;

    pub fn parse_command(line: &str) -> Input {
        let parts = line.trim().split(' ').collect::<Vec<&str>>();

        let command = parts[0].to_string();
        let target = parts[1].to_string();
        let mut args: Vec<String> = Vec::new();

        for i in 2..parts.len() {
            args.push(parts[i].to_string());
        }

        return Input::new(args, command, target);
    }

    pub fn base(input: &str) {
        let input_parsed = parse_command(input);
        let command = Command::get_command(&input_parsed);


        if input_parsed.target == command.valid_args[3] || input_parsed.args.contains(&command.valid_args[3]) {
            (command.help_fn)(&command);
            return;
        }

        let handle = command.handle_fn;
        handle(&command, &input_parsed);
    }
}