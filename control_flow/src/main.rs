fn main() {
    let num: i32 = 3;

    if num < 5 {
        println!("The number is smaller: {}", num);
    } else if num > 5 {
        println!("The number is greater: {}", num);
    } else {
        println!("The number is equal: {}", num);
    }

    let num: i32 = 0;

    if num != 0 {
        println!("The number is ok: {}", num);
    } else {
        println!("The number is zero");
    }

    let num: i32 = 6;

    if (num % 4) == 0 {
        println!("The number {} is divisible by 4", num);
    } else if (num % 3) == 0 {
        println!("The number {} is divisible by 3", num);
    } else if (num % 2) == 0 {
        println!("The number {} is divisible by 2", num);
    } else {
        println!("The number {} is not divisible by either 4 3 2", num);
    }

    //if in let statement

    let condition: bool = true;
    let num: i32 = if condition {
            5
        } else {
            6
        };

    println!("The value o f number is: {}", num);
    
}
