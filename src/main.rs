use std::env;
use std::process;
use std::thread;
use std::sync::mpsc::channel;

mod scan;


fn main(){
    let args: Vec<String> = env::args().collect();
    let f = args[0].clone();

    let arguments = scan::Arguments::new(&args).unwrap_or_else(
        |err| {
            if err.contains("help") {
                process::exit(0);
            } else {
                eprintln!("{} problem parsing arguments: {}", f, err);
                process::exit(0);
            }
        }
    );

    let threads = arguments.threads;
    let ip = arguments.ip;

    // Create a simple streaming channel
    let (tx, rx) = channel();

    for i in 0..threads{
        let tx = tx.clone();
        thread::spawn(move || {
            scan::scan(tx, i, ip, threads);
        });
    }

    let mut open_port = vec![];
    drop(tx);

    for port in rx{
        open_port.push(port);
    }

    open_port.sort();
    
    println!("Final List of open Ports: ");
    for port in open_port{
        println!("{} is OPEN!", port);
    }

}