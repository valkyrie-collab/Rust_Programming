// enum List {
//     Cons(i32, Box<List>),
//     Nil
// }
use std::rc::Rc;

enum List {
    Cons(i32, Rc<List>),
    Nil
}
use List::{Cons, Nil};
use std::mem::drop;

struct CustomerSmartPointer {
    data: String
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

impl Drop for CustomerSmartPointer {
    fn drop(&mut self) {
        println!("Dropping customer pointer with data {}", self.data);
    }
}

fn main() {
    // let b: Box<u32> = Box::new(5);
    // println!("b = {}", b);

    // let list: Box<List> = Box::new(Cons(1,Box::new(Cons(2, Nil))));

    let x: i32 = 5;
    let y: Box<i32> = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let m: Box<String> = Box::new(String::from("Rust"));
    hello(&m);

    let c: CustomerSmartPointer = CustomerSmartPointer { data: String::from("my stuff") };
    drop(c);
    let d: CustomerSmartPointer = CustomerSmartPointer { data: String::from("other stuff") };
    println!("Custom pointer created");

    let a: Rc<List> = Rc::new(Cons(3, Rc::new(Cons(2, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b: List = Cons(5, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let g: List = Cons(4, Rc::clone(&a));
        println!("count after creating g = {}", Rc::strong_count(&a));
    }

    println!("count after g goes out of scope = {}", Rc::strong_count(&a));
}
