use std::{env, process};
use better_grep::{Config, run};

fn main() {
    let _config: Config = Config::new(env::args()).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    println!("Searching for: {}", _config.query);
    println!("In file: {}", _config.filename);
    if let Err(e) = run(_config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    };
}