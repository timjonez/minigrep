use std::fs;
use std::env;
use std::error::Error;
use clap::Parser;
use colored::Colorize;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Config {
    /// string to search for
    #[clap()]
    pub query: String,

    /// file to search in
    #[clap()]
    pub filename: String,

    /// Search as case insensitive
    #[clap(short, long)]
    pub ignore_case: bool,
}

impl Config {
    pub fn new() -> Config {
        let mut config = Config::parse();
        if !config.ignore_case {
            config.ignore_case = !env::var("CASE_INSENSITIVE").is_err();
        }

        config
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<String> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            let matches: Vec<_> = line.match_indices(query).collect();
            let mut final_str = String::from(line);
            let len = query.len();
            for obj in &matches {
                let i = obj.0;
                final_str = format!("{}{}{}", &final_str[0..i], &final_str[i..i+len].red(), &final_str[i+len..final_str.len()]);
            }
            results.push(final_str);
        }
    }
    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str,) -> Vec<String> {
    let query = query.to_lowercase();
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            let lowercase = line.to_lowercase();
            let matches: Vec<_> = lowercase.match_indices(&query).collect();
            let mut final_str = String::from(line);
            let len = query.len();
            for obj in &matches {
                let i = obj.0;
                final_str = format!("{}{}{}", &final_str[0..i], &final_str[i..i+len].red(), &final_str[i+len..final_str.len()]);
            }
            results.push(final_str);
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "Rust:\nsafe, fast, productive.\nPick three.\nDuct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "Rust:\nsafe, fast, productive.\nPick three.\nTrust me.";
        assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, contents));
    }
}
