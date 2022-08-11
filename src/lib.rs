use std::{env, error::Error, fs};

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Missing arguments");
        }

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let file_content = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &file_content)
    } else {
        search(&config.query, &file_content)
    };

    for line in results {
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

pub fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();

    let mut result = vec![];

    for line in content.lines() {
        if line.to_lowercase().contains(&query) {
            result.push(line)
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "neat";
        let content = "\
As I mentioned
No one is beneath redemption.
Cool. Neat.";

        assert_eq!(
            vec!["No one is beneath redemption."],
            search(query, content)
        )
    }

    #[test]
    fn case_insensitive() {
        let query = "IoN";
        let content = "\
As I mentioned
No one is beneath redemption.
Cool. Neat.";

        assert_eq!(
            vec!["As I mentioned", "No one is beneath redemption."],
            search_case_insensitive(query, content)
        )
    }
}
