pub mod operate {
    use std::io::Split;
    use std::net::{IpAddr, Ipv4Addr};
    use std::process::{exit, Command as ProcessCommand};
    use std::str::FromStr;
    use crate::Operate::operate;
    use crate::Print::print;

    #[derive(Clone)]
    pub struct NetInfo {
        pub network_addr: Ipv4Addr,
        pub broadcast_addr: Ipv4Addr,
        pub prefix: u32,
        pub mask: u32,
        pub host_count: u32,
        pub first_host: Ipv4Addr,
        pub last_host: Ipv4Addr
    }

    impl NetInfo {
        pub fn new(network_addr: Ipv4Addr, broadcast_addr: Ipv4Addr, prefix: u32, mask: u32, host_count: u32, first_host: Ipv4Addr, last_host: Ipv4Addr) -> NetInfo {
            return NetInfo {
                network_addr,
                broadcast_addr,
                prefix,
                mask,
                host_count,
                first_host,
                last_host
            }
        }

        pub fn contains(&self, test_ip: &String) -> bool {
            let test_cidr = format!("{}/{}", test_ip, self.prefix);
            let test_info = get_network_info(&test_cidr);

            if test_info.is_err() {
                return false;
            }

            return self.network_addr == test_info.unwrap().network_addr;
        }
    }

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

    pub fn operate_netcalc(network_cidr: &String, info: bool, split: bool, split_value: &String, prefix: bool, prefix_value: &String, contains: bool, contains_value: &String) {
        if info {
            println!("------------------------------------------------------------------------------");
            let info = get_network_info(network_cidr);

            if info.is_err() {
                println!("{}", info.err().unwrap().to_string());
                return;
            }

            print::network_info(&info.ok().unwrap());
            println!("------------------------------------------------------------------------------");
        }

        if prefix {
            println!("------------------------------------------------------------------------------");
            let new_prefix = u32::from_str(prefix_value);

            if new_prefix.is_err() {
                println!("{}", new_prefix.err().unwrap().to_string());
                return;
            }

            let nets = split_by_prefix(network_cidr, new_prefix.ok().unwrap());

            if nets.is_err() {
                println!("{}", nets.err().unwrap().to_string());
                return;
            }

            print::network_info_list(nets.ok().unwrap());
            println!("------------------------------------------------------------------------------");
        }

        if split {
            println!("------------------------------------------------------------------------------");
            let count = u32::from_str(split_value);
            if count.is_err() {
                println!("{}", count.err().unwrap().to_string());
                return;
            }

            let nets = split_by_count(network_cidr, count.ok().unwrap());

            if nets.is_err() {
                println!("{}", nets.err().unwrap().to_string());
                return;
            }

            print::network_info_list(nets.ok().unwrap());
            println!("------------------------------------------------------------------------------");
        }

        if contains {
            println!("------------------------------------------------------------------------------");
            let info = get_network_info(network_cidr);
            let info_to_print = info.clone().unwrap();
            if info.is_err() {
                println!("{}", info.err().unwrap().to_string());
                return;
            }

            let result = info.ok().unwrap().contains(contains_value);

            match result {
                true => {println!("{}/{} contains {}", info_to_print.network_addr, info_to_print.prefix, contains_value)}
                false => {println!("{}/{} does not contain {}", info_to_print.network_addr, info_to_print.prefix, contains_value)}
            }
            println!("------------------------------------------------------------------------------");
        }
    }

    fn split_by_count(cidr: &String, count: u32) -> Result<Vec<String>, String> {
        for i in 0..count {
            if 1 << i >= count {
                let new_prefix = parse_cidr(cidr);

                if new_prefix.is_err() {
                    return Err(new_prefix.err().unwrap());
                }

                let (ip, prefix, mask) = new_prefix.ok().unwrap();
                return split_by_prefix(cidr, prefix + i);
            }
        }

        return Err(format!("{} not found", cidr));
    }

    pub fn get_network_info(cidr: &String) -> Result<NetInfo, String> {
        let cidr_result = parse_cidr(cidr);

        if cidr_result.is_err() {
            return Err(cidr_result.err().unwrap().to_string());
        }

        let (ip, prefix, mask) = cidr_result.ok().unwrap();
        let network_addr = get_network_addr(ip, mask);
        let broadcast_addr = get_broadcast_addr(ip, mask);
        let host_count = get_host_count(mask);
        let first_host = Ipv4Addr::from(u32::from(network_addr) + 1);
        let last_host = Ipv4Addr::from(u32::from(broadcast_addr) - 1);

        return Ok(NetInfo::new(network_addr, broadcast_addr, prefix, mask, host_count, first_host, last_host));
    }

    fn get_network_addr(ip: Ipv4Addr, mask: u32) -> Ipv4Addr {
        return Ipv4Addr::from(u32::from(ip) & mask);
    }

    fn get_broadcast_addr(ip: Ipv4Addr, mask: u32) -> Ipv4Addr {
        return Ipv4Addr::from(u32::from(ip) | !mask);
    }

    fn get_host_count(mask: u32) -> u32 {
        return (!mask).saturating_sub(1);
    }

    fn parse_cidr(cidr: &String) -> Result<(Ipv4Addr, u32, u32), String> {
        let Some((ip_str, prefix_str)) = cidr.split_once("/") else {
            return Err("Invalid format. Use network/prefix, 10.10.15.0/24".to_string());
        };

        let ip = Ipv4Addr::from_str(ip_str).unwrap();
        let prefix = u32::from_str(prefix_str).unwrap();


        let mut mask: u32 = 0;

        if prefix > 0 {
            mask = u32::MAX << (32 - prefix);
        }

        Ok((ip, prefix, mask))
    }

    fn split_by_prefix(cidr: &String, new_prefix: u32) -> Result<Vec<String>, String> {
        let cidr_result = parse_cidr(cidr);

        if cidr_result.is_err() {
            return Err(cidr_result.err().unwrap().to_string());
        }

        let (ip, prefix, mask) = cidr_result.ok().unwrap();

        if prefix >= new_prefix {
            return Err("New prefix must be greater than current prefix".to_string());
        }

        let net_count = 1 << (new_prefix - prefix);
        let size = 1 << (32 - new_prefix);
        let base_net = u32::from(ip) & mask;

        let mut cidrs: Vec<String> = Vec::new();

        for i in 0..net_count {
            let current = Ipv4Addr::from(base_net + (i * size as u32));
            let cidr = format!("{}/{}", current, new_prefix);
            cidrs.push(cidr);
        }

        return Ok(cidrs);
    }

    pub fn operate_trace(resolve: bool, hops: i32, timeout: i32, target: &String) {
        let max_hops = if hops > 0 { hops } else { 50 };
        let timeout = if timeout == 0 { 2000 } else { timeout };
        let mut ttl = 1;
        let mut reply_addr: Ipv4Addr = Ipv4Addr::from(u32::MIN);
        let target_addr_result = target.parse::<Ipv4Addr>();

        if target_addr_result.is_err() {
            println!("{} is not a valid IP!", target);
            return;
        }
        let target_addr = target_addr_result.unwrap();

        while target_addr != reply_addr && ttl <= max_hops {
            let mut ping_args = vec!["-n".to_string(), "1".to_string(), "-w".to_string(), timeout.to_string(), "-i".to_string()];
            ping_args.push(ttl.to_string());
            ping_args.push(target_addr.to_string());

            let output_option = run_and_capture("ping", &ping_args);

            if output_option == None {
                println!("Ping couldn't start!");
                return;
            }

            let output = output_option.unwrap();

            let reply_addr_option = find_reply_ip(&output);

            if reply_addr_option == None {
                println!("Couldn't parse reply IP from output!");
            }
            else {
                reply_addr = reply_addr_option.unwrap();
            }

            let mut hostname = String::new();
            if resolve {
                hostname = lookup(reply_addr).unwrap()
            }

            println!("{} | {}     TTL: {}", reply_addr, hostname, ttl.to_string());
            ttl += 1;
        }

        println!("Trace finished!");
    }

    fn lookup(ip: Ipv4Addr) -> Option<String> {
        let text = run_and_capture("nslookup", &[ip.to_string()])?;

        text.lines()
            .find_map(|line| line.trim_start().strip_prefix("Name:"))
            .map(|name| name.trim().to_string())
    }

    fn find_reply_ip(text: &String) -> Option<Ipv4Addr> {
        for line in text.lines() {
            if !(line.contains("Reply") || line.contains("Antwort") || line.contains("From") || line.contains("from")) {
                continue;
            }
            for word in line.split(|c: char| c == ' ' || c == ':' || c == '(' || c == ')') {
                if let Ok(ip) = word.parse::<Ipv4Addr>() {
                    return Some(ip);
                }
            }
        }
        None
    }

    fn run_and_capture(program: &str, args: &[String]) -> Option<String> {
        let output = ProcessCommand::new(program)
            .args(args)
            .output()
            .ok()?;

        Some(String::from_utf8_lossy(&output.stdout).to_string())
    }
    
    pub fn operate_exit() {
        exit(0);
    }
}