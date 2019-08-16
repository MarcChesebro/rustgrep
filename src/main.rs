use std::env;
use std::process;

use rustgrep::Config;

fn main() {

    //get arg variables
    let args: Vec<String> = env::args().collect();

    //create config struct
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    //run grep
    println!("query: {}\nfilename: {}", config.query, config.filename);
    if let Err(e) = rustgrep::run(config) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}

