use minigrep::Config;
use std::{env::args, process};

fn main() {
    println!("Welcome to minigrep a rust implementation of grep");

    let args: Vec<String> = args().collect();
    let config = Config::build(&args[..]).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    if let Err(e) = minigrep::run(config) {
        println!("Application Error: {e}");
        process::exit(1);
    }
}
