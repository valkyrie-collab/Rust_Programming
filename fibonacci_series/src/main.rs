use std::io::{self, Write};

fn give_fibonacci(mut range: i32) {
    let mut sum: i32;
    let mut x: i32 = 0;
    let mut y: i32 = 1;

    print!("{} -> {} -> ", x, y);

    while range > 0 {
        sum = x + y;
        x = y;
        y = sum;
        range -= 1;

        print!("{} -> ", sum);
    }

    println!("FIN");
}

fn main() {
    print!("Enter the range of fibonacci series: ");
    io::stdout().flush().expect("Cannot flush new line");

    let mut range: String = String::new();
    io::stdin().read_line(&mut range).expect("Cannot read");

    let range: i32 = match range.trim().parse() {
        Ok(n) => n,
        Err(_) => -1
    };

    if range <= -1 {
        println!("Range is not valid: {}", range);
    } else {
        give_fibonacci(range);
    }
}
