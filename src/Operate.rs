pub mod operate {
    use std::fs::FileTimes;
    use std::net::{Ipv4Addr, SocketAddr, ToSocketAddrs};
    use std::process::{exit, Command as ProcessCommand};
    use std::str::FromStr;
    use crate::Print::print;
    use std::time::{Duration, Instant};
    use std::net::TcpStream;

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

        pub fn contains(&self, test_ip: Ipv4Addr) -> bool {
            let test_cidr = format!("{}/{}", test_ip.to_string(), self.prefix);
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
            let info = get_network_info(network_cidr);

            if info.is_err() {
                println!("{}", info.err().unwrap().to_string());
                return;
            }

            println!("------------------------------------------------------------------------------");
            print::network_info(&info.ok().unwrap());
            println!("------------------------------------------------------------------------------");
        }

        if prefix {
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

            println!("------------------------------------------------------------------------------");
            print::network_info_list(nets.ok().unwrap());
            println!("------------------------------------------------------------------------------");
        }

        if split {
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

            println!("------------------------------------------------------------------------------");
            print::network_info_list(nets.ok().unwrap());
            println!("------------------------------------------------------------------------------");
        }

        if contains {
            let info = get_network_info(network_cidr);
            if info.is_err() {
                println!("{}", info.err().unwrap().to_string());
                return;
            }
            let info_to_print = info.clone().unwrap();

            let ip_to_check_result = get_ip_from_str(contains_value);

            if ip_to_check_result.is_err() {
                println!("{}", ip_to_check_result.err().unwrap().to_string());
                return;
            }

            let result = info_to_print.contains(ip_to_check_result.unwrap());

            println!("------------------------------------------------------------------------------");
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

        let ip = Ipv4Addr::from_str(ip_str).unwrap_or_else(|_| Ipv4Addr::UNSPECIFIED);
        let prefix = u32::from_str(prefix_str).unwrap_or_else(|_| u32::MAX);

        if ip == Ipv4Addr::UNSPECIFIED || prefix == u32::MAX {
            return Err("Invalid format. Use network/prefix, 10.10.15.0/24".to_string());
        }

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

        println!("Tracing {target_addr} (max {max_hops} hops, timeout {timeout}ms)");
        println!("{:<5} {:<18} {:<32} {}", "Hop", "Address", "Hostname", "Time");
        println!("{}", "-".repeat(66));
        while target_addr != reply_addr && ttl <= max_hops {
            let mut ping_args = vec!["-n".to_string(), "1".to_string(), "-w".to_string(), timeout.to_string(), "-i".to_string()];
            ping_args.push(ttl.to_string());
            ping_args.push(target_addr.to_string());

            let start = Instant::now();
            let output_option = run_and_capture("ping", &ping_args);
            let ms = start.elapsed().as_millis();

            if output_option == None {
                println!("Ping couldn't start!");
                return;
            }

            let output = output_option.unwrap();

            let Some(addr) = find_reply_ip(&output) else {
                println!("{:<5} {:<18} {:<32} {}", ttl, "*", "Request timed out", format!(">{timeout}ms"));
                ttl += 1;
                continue;
            };
            reply_addr = addr;

            let hostname = if resolve { lookup_hostname(reply_addr).unwrap_or("N/A".to_string()) } else { String::new() };

            println!("{:<5} {:<18} {:<32} {}", ttl, reply_addr.to_string(), hostname, format!("{ms}ms"));
            ttl += 1;
        }

        println!("Trace finished!");
    }

    fn lookup_hostname(ip: Ipv4Addr) -> Option<String> {
        let text = run_and_capture("nslookup", &[ip.to_string()])?;

        let name = text.lines().find_map(|line| line.trim_start().strip_prefix("Name:"))?;

        Some(name.trim().to_string())
    }

    pub fn lookup_addr(hostname: &String) -> Option<Ipv4Addr> {
        let text = run_and_capture("nslookup", &[hostname.to_string()])?;

         let addr = text.lines().skip_while(|l| !l.contains("Addresses:"))
            .skip(1)
            .find_map(|line| {
                let trimmed = line.trim();
                trimmed.parse::<Ipv4Addr>().ok()
            });

        match addr {
            Some(addr) => Some(addr),
            None => None,
        }
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
            .stdin(std::process::Stdio::inherit())
            .output()
            .ok()?;

        let mut text = String::from_utf8_lossy(&output.stdout).into_owned();
        text.push_str(&String::from_utf8_lossy(&output.stderr));

        Some(text)
    }

    const COMMON_PORTS: &[u16] = &[21, 22, 23, 25, 53, 80, 110, 135, 139, 143, 443, 445, 1433, 3306, 3389, 5432, 5900, 8080, 8443];

    pub fn operate_portscan(ports: Vec<u16>, range: [u32; 2], common: bool, timeout: u64, target: Ipv4Addr) {
        let mut open_ports = 0;
        let mut closed_ports = 0;
        let mut ports_empty = false;
        println!("{:<6} | {:<15} | {:<10} | {}", "Port", "Target", "Status", "Info");
        println!("{}", "-".repeat(66));
        if ports.is_empty() {ports_empty = true;}
        else {
            for port in ports {
                let result = test_port(target, timeout, port);

                match result {
                    Ok(output) => {println!("{}", output); open_ports += 1;}
                    Err(error) => {println!("{}", error); closed_ports += 1;}
                }
            }
        }

        if common == false && ports_empty {
            for i  in range[0]..range[1]+1 {
                let result = test_port(target, timeout, i as u16);

                match result {
                    Ok(output) => {println!("{}", output); open_ports += 1;}
                    Err(error) => {println!("{}", error); closed_ports += 1;}
                }
            }
        }
        else if common == true && ports_empty {
            for port in COMMON_PORTS {
                let result = test_port(target, timeout, *port);

                match result {
                    Ok(output) => {println!("{}", output); open_ports += 1;}
                    Err(error) => {println!("{}", error); closed_ports += 1;}
                }
            }
        }

        println!("Scan on target {} finished!", target);
        println!("{} open ports", open_ports);
        println!("{} closed ports", closed_ports);
        println!("{}", "-".repeat(66));
    }

    fn test_port (target: Ipv4Addr, timeout: u64, port: u16) -> Result<String, String> {
        let socket_addr = (target, port).to_socket_addrs().unwrap().next();

        if socket_addr.is_none() {
            println!("Couldn't test port {} on target {}", port, target);
        }

        let duration = Duration::from_millis(timeout);
        let result = TcpStream::connect_timeout(&socket_addr.unwrap(), duration);

        match result {
            Ok(_stream) => Ok(format!("{:<6} | {:<15} | {:<7} | {}", port, target, "open", "")),
            Err(err) => Err(format!("{:<6} | {:<15} | {:<7} | {}", port, target, "closed", err)),
        }
    }

    pub fn operate_exit() {
        exit(0);
    }

    pub fn get_ip_from_str(str: &String) -> Result<Ipv4Addr, String> {
        let ip = str.parse::<Ipv4Addr>();

        if ip.is_err() {
            return Err(ip.err().unwrap().to_string());
        }

        Ok(ip.unwrap())
    }
}