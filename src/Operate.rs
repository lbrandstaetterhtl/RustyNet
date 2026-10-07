pub mod operate {
    use std::process::{exit, Command as ProcessCommand};

    pub fn operate_ping(count: Option<u32>, size: Option<u32>, timeout: Option<u32>, target: &String) {
        let mut args: Vec<String> = Vec::new();

        if let Some(c) = count {
            args.push("-n".to_string());
            args.push(c.to_string());
        }

        if let Some(s) = size {
            args.push("-l".to_string());
            args.push(s.to_string());
        }

        if let Some(t) = timeout {
            args.push("-w".to_string());
            args.push(t.to_string());
        }

        args.push(target.to_string());

        let status = ProcessCommand::new("ping")
            .args(&args)
            .status()                     
            .expect("ping couldn't be executed");
    }
    
    pub fn operate_exit() {
        exit(0);
    }
}