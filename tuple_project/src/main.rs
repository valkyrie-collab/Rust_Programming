fn main() {
    let tup: (i32, f64, u8) = (500, 78.89, 1);
    let (x, y, z): (i32, f64, u8) = tup;

    println!("The value of each x: {}, y: {}, z: {}", x, y, z);

    //accessing value
    let first_value: i32 = tup.0;
    let second_value: f64 = tup.1;
    let third_value: u8 = tup.2;

    println!("The value of first value: {}, second value: {}, third value: {}", first_value, second_value, third_value);
    
}
