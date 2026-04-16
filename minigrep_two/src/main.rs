// extern crate minigrep_two; not used in this days 
use std::{env, process};
use minigrep_two::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config: Config = Config::new(&args).unwrap_or_else(|err| {
        println!("ther error is {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep_two::run(config) {
        println!("new error in run function: {}", e);
        process::exit(1);
    }
}
