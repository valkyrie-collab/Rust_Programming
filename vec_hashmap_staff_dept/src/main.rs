use std::io::{self, Write};
use std::collections::HashMap;

fn main() {
    print!("Enter the size of staff engineering: ");
    io::stdout().flush().expect("Cannot clean new line");

    let mut len_one: String = String::new();
    io::stdin().read_line(&mut len_one).expect("cannot read line");

    let len_one: usize = match len_one.trim().parse() {
        Ok(n) => n,
        Err(_) => 0
    };

    print!("Enter the size of staff sales");
    io::stdout().flush().expect("Cannot clean the new line");

    let mut len_two: String = String::new();
    io::stdin().read_line(&mut len_two).expect("cannot read line");

    let len_two: usize = match len_two.trim().parse() {
        Ok(n) => n,
        Err(_) => 0
    };    

    let mut staff_one: Vec<String> = vec![String::new(); len_one];
    let mut staff_two: Vec<String> = vec![String::new(); len_two];

    let mut i: usize = 0;

    while i < len_one {
        print!("Enter the name of staff: ");
        io::stdout().flush().expect("Cannot flush line");
        io::stdin().read_line(&mut staff_one[i]).expect("Cannot readline");
        i += 1;
    }

    i = 0;

    while i < len_two {
        print!("Enter the name of staff: ");
        io::stdout().flush().expect("Cannot flush line");
        io::stdin().read_line(&mut staff_two[i]).expect("Cannot readline");
        i += 1;
    }

    let mut map: HashMap<String, Vec<String>> = HashMap::new();

    map.insert(String::from("Engineering"), staff_one);
    map.insert(String::from("Sales"), staff_two);

    if let Some(i) = map.get("Engineering") {

        for k in i {
            print!("{}", k);
        }

    }

    if let Some(i) = map.get("Sales") {

        for k in i {
            print!("{}", k);
        }

    }

}
