use std::io::{self, Write};

fn c_to_f(temp: f32) -> f32 {
    let fah: f32 = 1.8 * temp + 32.0;
    fah
}

fn f_to_c(temp: f32) -> f32 {
    let cel: f32 = 5.0 / 9.0 * (temp - 32.0);
    cel
}

fn main() {
    print!("Enter the Temperature: ");
    io::stdout().flush().expect("Cannot flush new line");

    let mut temp: String = String::new();
    io::stdin().read_line(&mut temp).expect("Cannot read the value from readline");

    let temp: f32 = match temp.trim().parse() {
        Ok(n) => n,
        Err(_) => -1.1
    };

    println!("value of temp: {}", temp);

    let fah: f32 = c_to_f(temp);
    let cel: f32 = f_to_c(temp);

    println!("The value of fahrenhite: {}", fah);
    println!("The value of celceius: {}", cel);
}
