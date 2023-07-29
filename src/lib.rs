use std::{env, error::Error, fs, vec};

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Self, &'static str> {
        if args.len() < 3 {
            return Err("Args not valid");
        }

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query: args[1].clone(),
            file_path: args[2].clone(),
            ignore_case,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.file_path)?;

    // do something with content
    for line in if config.ignore_case {
        search_case_insensitive(&config.query, &content)
    } else {
        search(&config.query, &content)
    } {
        println!("{line}");
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut res: Vec<&str> = vec![];
    for line in contents.lines() {
        if line.contains(query) {
            res.push(line)
        }
    }
    res
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut res: Vec<&str> = vec![];
    let query = query.to_lowercase();
    for line in contents.lines() {
        let line_lower = line.to_lowercase();
        if line_lower.contains(&query) {
            res.push(line)
        }
    }
    res
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
