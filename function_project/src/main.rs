fn another_function(x: i32, y: i32) {
    println!("This is the another function {}", x);

    println!("The value of x: {}", x);
    println!("The value of y: {}", y);
}

fn return_function(x: i32) -> i32 {
    5 + x
}

fn main() {
    println!("Hello, world!");

    another_function(4, 7);

    let x: i32 = 5;
    let y: i32 = {
        let x: i32 = 3;
        x + 1
    };

    println!("The value of main x: {}", x);
    println!("The value of main y: {}", y);

    let num: i32 = return_function(5);
    println!("The return value: {}", num);
}
