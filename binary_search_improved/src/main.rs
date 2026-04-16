extern crate binary_search_improved;
use binary_search_improved::search::{binary, binary_rec, linear};

fn main() {
    let arr: [u32; 5] = [1, 2, 3, 4, 5];

    let mut val: i32 = binary::search(&arr, 0, 4, 5);
    println!("The value of val1 = {}", val);

    val = binary_rec::search(&arr, 0, 4, 1);
    println!("The value of val2 = {}", val);

    val = linear::search(&arr, 5);
    println!("The value of val3 = {}", val);
}
