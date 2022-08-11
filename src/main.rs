use std::{env, fs, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("{err}");
        process::exit(1)
    });

    println!("Search query: {}", config.query);
    println!("File (path): {}", config.file_path);

    let file_content = fs::read_to_string(config.file_path).expect("Could not read file");

    println!("File content:\n{file_content}");
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Missing arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}
