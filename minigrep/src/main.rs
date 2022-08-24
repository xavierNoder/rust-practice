use minigrep::Config;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problen parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Search for {}", config.query);
    println!("IN file {}", config.filename);
    if let Err(e) = minigrep::run(config) {
        println!("Application error:{}", e);
        process::exit(1);
    }
}
