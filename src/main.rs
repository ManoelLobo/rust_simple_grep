use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let file_path = &args[2];

    println!("Search query: {query}");
    println!("File (path): {file_path}");
}
