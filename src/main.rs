use std::{env, process};

use simplegrep::Config;

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Argument error: {err}");
        process::exit(1);
    });

    if let Err(e) = simplegrep::run(config) {
        eprintln!("App error: {e}");
        process::exit(1);
    };
}
