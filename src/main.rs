use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = parse_config(&args);

    println!("Search query: {}", config.query);
    println!("File (path): {}", config.file_path);

    let file_content = fs::read_to_string(config.file_path).expect("Could not read file");

    println!("File content:\n{file_content}");
}

struct Config {
    query: String,
    file_path: String,
}

fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let file_path = args[2].clone();

    Config { query, file_path }
}
