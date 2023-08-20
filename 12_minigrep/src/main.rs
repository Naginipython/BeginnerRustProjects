use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    // dbg!(args);

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for: {}", config.query);
    println!("In file: {}", config.file_path);

    //We use 'if let' rather than 'unwrap_or_else' to check whether "run" returns an Err value and call process::exit(1) if it does
    if let Err(e) = minigrep::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}

