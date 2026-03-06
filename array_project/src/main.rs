fn main() {
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    let months: [&str; 12] = ["January", "February", "March", "April",
        "May", "June", "July", "August", "September", "October", "November", "December"];

    let first: (i32, &str) = (arr[0], months[0]);
    let second: (i32, &str) = (arr[1], months[1]);

    println!("The first value {} {}", first.0, first.1);
    println!("The second value {} {}", second.0, second.1);

    // let invalid_access: i32 = arr[6];
    // println!("The invalid access: {}", invalid_access);
}
