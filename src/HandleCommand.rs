pub mod command {
    use std::collections::HashMap;
    use crate::Operate::operate;
    use crate::Print::print;

    #[derive(Clone)]
    pub struct Command {
        pub valid_args: Vec<String>,
        pub command: String,
        pub handle_fn: fn(&Command, &Input),
        pub help_fn: fn(&Command)
    }

    impl Command {

        pub fn get_command_list() -> HashMap<String, Command> {
            let ping_args = vec!["--size".to_string(), "--timeout".to_string(), "--count".to_string()];
            let ping: Command = Command::new("ping".to_string(), ping_args, Command::handle_ping, Command::help_ping);

            let mut null_args = Vec::new();
            let help: Command = Command::new("help".to_string(), null_args, Command::handle_help, Command::help_all);

            null_args = Vec::new();
            let exit: Command = Command::new("exit".to_string(), null_args, Command::handle_exit, Command::help_all);

            null_args = Vec::new();
            let clear: Command = Command::new("clear".to_string(), null_args, Command::handle_clear, Command::help_all);

            let netcalc_args = vec!["--info".to_string(), "--split".to_string(), "--contains".to_string(), "--prefix".to_string()];
            let netcalc: Command = Command::new("netcalc".to_string(), netcalc_args, Command::handle_netcalc, Command::help_netcalc);

            let trace_args = vec!["--ttl".to_string(), "--timeout".to_string(), "--resolve".to_string()];
            let trace: Command = Command::new("trace".to_string(), trace_args, Command::handle_trace, Command::help_trace);

            let mut commands: HashMap<String, Command> = HashMap::new();

            commands.insert("ping".to_string(), ping);
            commands.insert("help".to_string(), help);
            commands.insert("exit".to_string(), exit);
            commands.insert("clear".to_string(), clear);
            commands.insert("netcalc".to_string(), netcalc);
            commands.insert("trace".to_string(), trace);

            return commands;
        }

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

        pub fn handle_netcalc(&self, input: &Input) {
            let mut info = false;
            let mut split = false;
            let mut split_value = String::new();
            let mut contains = false;
            let mut contains_value = String::new();
            let mut prefix = false;
            let mut prefix_value = String::new();

            let mut i = 0;
            while i < input.args.len() {
                if input.args[i] == self.valid_args[0] {
                    info = true;
                }

                if input.args[i] == self.valid_args[1] {
                    split = true;
                    split_value = input.args[i + 1].trim().to_string();
                    i += 1;
                }

                if input.args[i] == self.valid_args[2] {
                    contains = true;
                    contains_value = input.args[i + 1].trim().to_string();
                    i += 1;
                }

                if input.args[i] == self.valid_args[3] {
                    prefix = true;
                    prefix_value = input.args[i + 1].trim().to_string();
                    i += 1;
                }

                i += 1;
            }

            operate::operate_netcalc(&input.target,info, split, &split_value, prefix, &prefix_value, contains, &contains_value);
        }

        pub fn handle_help(&self, _input: &Input) {
            let help = self.help_fn;
            help(self)
        }

        pub fn handle_exit(&self, _input: &Input) {
            clearscreen::clear().expect("Failed to clearscreen");
            operate::operate_exit();
        }

        pub fn handle_clear(&self, _input: &Input) {
            clearscreen::clear().expect("Failed to clearscreen");
            print::header();
        }

        pub fn handle_trace(&self, input: &Input) {
            let mut resolve = false;
            let mut hops = 0;
            let mut timeout = 0;

            let mut i = 0;
            while i < input.args.len() {
                if input.args[i] == self.valid_args[0] {
                    hops = input.args[i + 1].trim().parse::<i32>().unwrap();
                    i += 1;
                }

                if input.args[i] == self.valid_args[1] {
                    timeout = input.args[i + 1].trim().parse::<i32>().unwrap();
                    i += 1;
                }

                else if input.args[i] == self.valid_args[2] {
                    resolve = true;
                }

                i += 1;
            }

            operate::operate_trace(resolve, hops, timeout, &input.target);
        }

        pub fn new(command: String, valid_args: Vec<String>, handle_fn: fn(&Command, &Input), help_fn: fn(&Command)) -> Command {
           return Command {
               valid_args,
               command,
               handle_fn,
               help_fn
           };
        }

        pub fn get_command(input: &Input) -> Command {
            let commands = Command::get_command_list();

            return match commands.get(&input.command) {
                Some(command) => command.clone(),
                None => {commands["help"].clone()}
            }
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
    }
}

pub mod handle {
    use crate::HandleCommand::command::Command;
    use crate::HandleCommand::command::Input;

    pub fn base(input: &str) {
        let input_parsed = Input::parse_input(input.trim());
        let command = Command::get_command(&input_parsed);


        if command.valid_args.len() == 0 {
            let handle = command.handle_fn;
            handle(&command, &input_parsed);
            return;
        }

        let is_help = input_parsed.target == "--h" || input_parsed.args.iter().any(|a| a == "--h");

        if is_help {
            (command.help_fn)(&command);
            return;
        }

        let handle = command.handle_fn;
        handle(&command, &input_parsed);
    }
}