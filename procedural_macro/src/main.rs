extern crate procedural_macro;

use procedural_macro::MyDerive;

#[derive(MyDerive)]
struct Person {
    name: String,
    age: usize
}

fn main() {
    let p: Person = Person {name: String::from("Rajarshi"), age: 22};
    println!("{}", p);
}