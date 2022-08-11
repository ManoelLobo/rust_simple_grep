use std::{env, process};

use simplegrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("{err}");
        process::exit(1)
    });

    println!("Search query: {}", config.query);
    println!("File (path): {}", config.file_path);

    if let Err(e) = simplegrep::run(config) {
        println!("App error: {e}");

        process::exit(1);
    };
}
