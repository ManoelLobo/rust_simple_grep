use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let file_path = &args[2];

    println!("Search query: {query}");
    println!("File (path): {file_path}");

    let file_content = fs::read_to_string(file_path).expect("Could not read file");

    println!("File content:\n{file_content}");
}
