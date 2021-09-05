use std::net::IpAddr;
use std::str::FromStr;

pub struct Arguments {
    pub flag: String,
    pub ip: IpAddr,
    pub threads: u16
}

impl Arguments{
    pub fn new(args: &[String]) -> Result<Arguments, &'static str>{

        let f = args[1].clone();
        if let Ok(ip) = IpAddr::from_str(&f){
            // when only ip is passed, scan with 4 threads
            return Ok(Arguments {flag: String::from(""), ip, threads: 4});
        }else{
            let flag = args[1].clone();
            if flag.contains("-h") || flag.contains("--help") && args.len() == 2 {
                // help function
                println!("Usage: ");
                return Err("help");
            }else if flag.contains("-h") || flag.contains("--help"){
                // if help function is passed along with other arguments
                return Err("too many arguments");
            }else if flag.contains("-t"){
                // when custom number of thread are passed
                let ip = match IpAddr::from_str(&args[3]){
                    Ok(s) => s,
                    Err(_) => return Err("not a valid IP address")
                };
                let threads = match args[2].parse::<u16>(){
                    Ok(s) => s,
                    Err(_) => return Err("failed due to thread count")
                };
                return Ok(Arguments{threads, flag, ip});
            }else{
                return Err("invalid synatx");
            }
        } 
    }
}
