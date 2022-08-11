use std::{error::Error, fs};

pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Missing arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let file_content = fs::read_to_string(config.file_path)?;

    for line in search(&config.query, &file_content) {
        println!("{line}")
    }

    Ok(())
}

pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut result = vec![];

    for line in content.lines() {
        if line.contains(query) {
            result.push(line)
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn any_result() {
        let query = "neat";
        let content = "\
As I mentioned
No one is beneath redemption.
Cool.";

        assert_eq!(
            vec!["No one is beneath redemption."],
            search(query, content)
        )
    }
}
