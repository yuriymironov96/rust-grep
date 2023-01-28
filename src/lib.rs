use std::env;
use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.haystack)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.needle, &contents)
    } else {
        search(&config.needle, &contents)
    };

    for line in results {
        println!("{line}")
    }

    Ok(())
}

pub struct Config {
    needle: String,
    haystack: String,
    ignore_case: bool,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        // skip irrelevant first element
        args.next();

        let needle = match args.next() {
            Some(arg) => arg,
            None => return Err("No query"),
        };

        let haystack = match args.next() {
            Some(arg) => arg,
            None => return Err("No file"),
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            needle,
            haystack,
            ignore_case,
        })
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
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
        let query = "duct";
        let contents = "\
        Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents))
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
        Rust:
safe, fast, productive.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        )
    }
}
