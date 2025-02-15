use minigrep::Config;
use std::{env::args, process};

fn main() {
    println!("Welcome to minigrep a rust implementation of grep");

    let config = Config::build(std::env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application Error: {e}");
        process::exit(1);
    }
}
