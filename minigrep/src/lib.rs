use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::error::Error;

pub struct Config {
    query: String,
    filename: String,
    case_sensitive: bool
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> { 

        if args.len() < 3 {
            // panic!("not enough arguments");
            return Err("not enough arguments");
        } 

        let case_sensitive: bool = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query: args[1].clone(),
            filename: args[2].clone(),
            case_sensitive: case_sensitive
        })
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // //            ^^^
    // //            Lifetime parameter
    
    let mut results = Vec::new();
    
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    
    results
    // vec![]
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query: String = query.to_lowercase();
    let mut results: Vec<&'a str> = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // println!("Searching for {}", config.query);
    // println!("In the file {}", config.filename);

    let mut file: File = File::open(config.filename)?;
    let mut contents: String = String::new();

    file.read_to_string(&mut contents)?;

    let result: Vec<&str> = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for lines in result {
        println!("{}", lines);
    }

    Ok(())

    // println!("With text:\n\n{}", contents);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query: &str = "duct";
        let contents: &str = "Rust:\nsafe, fast, productive.\nPick three.\nDuct tape.";
        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
    }

    #[test]
    fn case_insensitive() {
        let query: &str = "rUsT";
        let contents: &str = "Rust:\nsafe, fast, productive\npick three\nTrust me.";
        assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, contents));
    }
}