use std::{env, process};

use simplegrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Argument error: {err}");
        process::exit(1);
    });

    if let Err(e) = simplegrep::run(config) {
        println!("App error: {e}");
        process::exit(1);
    };
}
