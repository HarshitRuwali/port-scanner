use std::net::{IpAddr, TcpStream};
use std::str::FromStr;
use std::sync::mpsc::Sender;
use std::io::{self, Write};

// const MAX: u16 = 65535;

pub struct Arguments {
    pub flag: String,
    pub ip: IpAddr,
    pub threads: u16,
    pub total_ports: u16
}

impl Arguments{
    pub fn new(args: &[String]) -> Result<Arguments, &'static str>{

        let f = args[1].clone();
        if let Ok(ip) = IpAddr::from_str(&f){
            // when only ip is passed, scan with 4 threads
            return Ok(Arguments {flag: String::from(""), ip, threads: 4, total_ports:1000});
        }else{
            let flag = args[1].clone();
            if flag.contains("-h") || flag.contains("--help") && args.len() == 2 {
                // help function
                println!("
                    Port Scanner Made using Rust \n
                    Usage: port-scanner [-h] [-t thread_count] [ip] \n
                    where: \n
                        -h  show this help text \n
                        -t  set the thread count \n
                        -p- to scan all open ports \n
                ");
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
                return Ok(Arguments{threads, flag, ip, total_ports:1000});
            }else if flag.contains("-p-") {
                let ip = match IpAddr::from_str(&args[3]){
                    Ok(s) => s,
                    Err(_) => return Err("not a valid IP address")
                };
                let threads = match args[2].parse::<u16>(){
                    Ok(s) => s,
                    Err(_) => return Err("failed due to thread count")
                };
                return Ok(Arguments{threads, flag, ip, total_ports: 65535});
            }else{
                return Err("invalid synatx");
            }
        } 
    }
}


pub fn scan(tx: Sender<u16>, start_port: u16, ip: IpAddr, threads: u16, total_ports: u16){

    let mut port:u16 = start_port + 1;

    loop{
        match TcpStream::connect((ip, port)){
            Ok(_) => {
                println!("Found Open port : {}", port);
                io::stdout().flush().unwrap();
                tx.send(port).unwrap();
            }
            Err(_) => {}
        }
        if(total_ports - port) <= threads{
            break;
        }
        port += threads;
    }
}