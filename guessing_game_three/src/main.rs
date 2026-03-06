extern crate rand;

use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main() {
    println!("Enter the number...");
    println!("please input your guess.");

    let num = rand::thread_rng().gen_range(1..=100);

    loop {
        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Cannot read from terminal");

        let guess: i32 = match guess.trim().parse() {
            Ok(n) => n,
            Err(_) => continue,
        };
        println!("you guessed: {}", guess);
        println!("The secret number is: {}", num);

        match guess.cmp(&num) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too Big"),
            Ordering::Equal => {
                println!("you win!");
                break;
            }
        }
    }
}
