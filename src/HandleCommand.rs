pub mod command {
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
                    println!("{} is not a valid argument", input.args[i]);
                }
                i += 1;
            }

            operate::operate_ping(count, size, timeout, &input.target);
        }

        pub fn handle_help(&self, _input: &Input)
        {
            let help = self.help_fn;
            help(self)
        }

        pub fn handle_exit(&self, _input: &Input)
        {
            operate::operate_exit();
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

            let mut null_args = Vec::new();
            let help: Command = Command::new("help".to_string(), null_args, Command::handle_help, Command::help_all);

            null_args = Vec::new();
            let exit: Command = Command::new("exit".to_string(), null_args, Command::handle_exit, Command::help_all);

            if input.command == exit.command {
                return exit;
            }
            else if input.command == help.command {
                return help;
            }
            else if input.command == ping.command {
                return ping;
            }
            return help;
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
    use std::process::exit;
    use crate::HandleCommand::command::Command;
    use crate::HandleCommand::command::Input;

    pub fn parse_input(line: &str) -> Input {
        let parts = line.trim().split(' ').collect::<Vec<&str>>();

        if parts.len() == 1 {
            let null_args = Vec::new();
            return Input::new(null_args, parts[0].to_string(), " ".to_string());
        }

        let command = parts[0].to_string();
        let target = parts[1].to_string();
        let mut args: Vec<String> = Vec::new();

        for i in 2..parts.len() {
            args.push(parts[i].to_string());
        }

        return Input::new(args, command, target);
    }

    pub fn base(input: &str) {
        let input_parsed = parse_input(input);
        let command = Command::get_command(&input_parsed);


        if command.valid_args.len() == 0 {
            let handle = command.handle_fn;
            handle(&command, &input_parsed);
            return;
        }
        else if input_parsed.target == command.valid_args[3] || input_parsed.args.contains(&command.valid_args[3]) {
            (command.help_fn)(&command);
            return;
        }

        let handle = command.handle_fn;
        handle(&command, &input_parsed);
    }
}