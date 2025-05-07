use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
    // query: String,
    // file_path: String,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Self, &'static str> {
        args.next();
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string")
        };
        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        Ok(Self {
            query,
            file_path,
            ignore_case,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    let results = match config.ignore_case {
        false => search(config.query, &contents),
        true => search_case_insensitive(config.query, &contents),
    }; // Poderia usar if/else aqui.
    for line in results {
        println!("{line}")
    }
    Ok(())
}

pub fn search(query: String, contents: &str) -> Vec<&str> {
    contents
        .lines()
        .filter(|line| line.contains(&query))
        .collect()
}

pub fn search_case_insensitive(query: String, contents: &str) -> Vec<&str> {
    let query = query.to_lowercase();
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = String::from("duct");
        let contents = "\
Rust:
safe, fast, productive.
Pic three.
Duct tape.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = String::from("rUsT");
        let contents = "\
Rust:
safe, fast, productive.
Pic three.
Trust me.";
        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
