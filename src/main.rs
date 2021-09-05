use std::env;
use std::process;

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

}