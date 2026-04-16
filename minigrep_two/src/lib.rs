use std::env;
use std::fs::File;
use std::io::Read;

pub struct Config {
    query: String,
    filename: String,
    sensitive: bool
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {

        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        let sensitive: bool = env::var("SENSE").is_err();

        Ok(
            Config {
                query: args[1].clone(),
                filename: args[2].clone(),
                sensitive
            }
        )
    }
}

fn search_case_sensitive<'a>(conf_query: &String, cnt: &'a String) -> Vec<&'a str> {
    let mut file_cnts: Vec<&'a str> = Vec::new();

    for line in cnt.lines() {

        if line.contains(conf_query) {
            file_cnts.push(line);
        }

    }

    file_cnts
}

fn search_case_insensitive<'a>(conf_query: &String, cnt: &'a String) -> Vec<&'a str> {
    let mut file_cnts: Vec<&'a str> = Vec::new();

    for line in cnt.lines() {
        let temp: String = line.to_lowercase();

        if temp.contains(&conf_query.to_lowercase()) {
            file_cnts.push(line);
        }

    }

    file_cnts
}

pub fn run(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    let mut file: File = File::open(config.filename)?;
    let mut content: String = String::new();

    file.read_to_string(&mut content)?;

    let result: Vec<&str> = if config.sensitive {
        search_case_sensitive(&config.query, &content)
    } else {
        search_case_insensitive(&config.query, &content)
    };

    for lines in result {
        println!("{}", lines);
    }

    Ok(())
}