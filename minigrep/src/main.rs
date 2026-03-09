extern crate minigrep;
use std::{env, process};
use minigrep::Config;
// use std::fs::File;
// use std::io::prelude::*;
// use std::error::Error;

// struct Config {
//     query: String,
//     filename: String
// }

// impl Config {
//     fn new(args: &[String]) -> Result<Config, &'static str> { 

//         if args.len() < 3 {
//             // panic!("not enough arguments");
//             return Err("not enough arguments");
//         }

//         Ok(Config {
//             query: args[1].clone(),
//             filename: args[2].clone()
//         })

//     }
// }

// fn run(config: Config) -> Result<(), Box<dyn Error>> {
//     // println!("Searching for {}", config.query);
//     // println!("In the file {}", config.filename);

//     let mut file: File = File::open(config.filename)?;
//     let mut contents: String = String::new();

//     file.read_to_string(&mut contents)?;

//     Ok(())

//     // println!("With text:\n\n{}", contents);
// }

fn main() {
    let args: Vec<String> = env::args().collect();
    // println!("{:?}", args);
    // let query: &String = &args[1];
    // let filename: &String = &args[2];
    // let (query, filename): (&str, &str) = parse_config(&args);
    
    

    let config: Config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error {}", e);
        
        process::exit(1);
    }
}
