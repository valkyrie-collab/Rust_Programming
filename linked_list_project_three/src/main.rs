mod lists;
use lists::single::SingleList;
use lists::double::DoubleList;
use lists::traits::ListOp;
use std::io::{self, Write};

fn do_list<T: ListOp>(list: &mut T) {
    print!("Enter the list: ");
    io::stdout().flush().expect("Cannot flush new line");

    let mut wrd: String = String::new();
    io::stdin().read_line(&mut wrd).expect("Cannot read data from input");

    let mut num: i32 = 0;

    for i in wrd.chars() {
        
        if i.is_digit(10) {
            num = num * 10 + ((i as i32) - 48);
        } else {
            list.append(num);
            num = 0;
        }

    }

}

fn main() {
    let mut single_list: SingleList = lists::new_single_list();
    let mut double_list: DoubleList = lists::new_double_list();
    
    do_list(&mut single_list);
    do_list(&mut double_list);
    
    single_list.show_list();
    double_list.show_list();
}
