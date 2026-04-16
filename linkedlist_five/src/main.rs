extern crate linkedlist_five;

use linkedlist_five::linkedlist::{single::Single, double::Double};

fn main() {
    let mut single_list: Single = Single::new();
    let mut double_list: Double = Double::new();
    
    let value_one: String = String::from("hello");
    let value_two: String = String::from("world");
    let value_three: String = String::from("this is");
    let value_four: String = String::from("rust");
    
    single_list.append_list(value_one.clone());
    single_list.append_list(value_two.clone());
    single_list.append_list(value_three.clone());
    single_list.append_list(value_four.clone());
    
    double_list.append_next(value_one);
    double_list.append_next(value_two);
    double_list.append_next(value_three);
    double_list.append_next(value_four);
    
    single_list.show_list();
    double_list.show_list();
}
