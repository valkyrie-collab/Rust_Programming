extern crate rand;

use std::io::{self, Write};
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let num: i32 = rand::thread_rng().gen_range(1..=100);

    loop {
        print!("Enter the number: ");
        io::stdout().flush().expect("Cannot flush new line");
        
        let mut guess: String = String::new();
        io::stdin().read_line(&mut guess).expect("Cannot read the value");

        let guess: i32 = match guess.trim().parse() {
            Ok(n) => n,
            Err(_) => continue,
        };

        println!("The number you guess... {}", guess);
        println!("The secret number is... {}", num);

        match guess.cmp(&num) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too Big"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
        }

    }
    
}
