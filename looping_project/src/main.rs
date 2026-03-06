fn main() {
    let mut number: i32 = 3;

    while number != 0 {
        println!("{}", number);
        number -= 1;
    }

    println!("LIFTOFF!!");

    let mut number: i32 = 4;

    loop {
        println!("{}", number);
        number -= 1;

        if number == 0 {
            break;
        }

    }

    let arr: [i32; 5] = [10, 20, 30, 40, 50];
    let mut index: usize = 0;

    while index < 5 {
        println!("The value is: {}", arr[index]);
        index += 1;
    }

    for i in arr.iter() {
        println!("For, The value is: {}", i);
    }

    println!("First");
    for i in 1..5 {
        println!("{}", i);
    }

    println!("Second");
    for i in (1..5).rev() {
        println!("{}", i);
    }

}
