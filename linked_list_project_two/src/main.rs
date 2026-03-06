use std::io::{self, Write};

struct Node {
    value: i32,
    next: Option<Box<Node>>
}

struct List {
    head: Option<Box<Node>>
}

impl Node {

    fn new(value: i32) -> Node {
        Node {
            value: value,
            next: None
        }
    } 

}

impl List {

    fn new() -> Self {
        List {head: None}
    }

    fn append(&mut self, value: i32) {
        let new_node: Box<Node> = Box::new(Node::new(value));

        match self.head.as_mut() {
            None => {
                self.head = Some(new_node);
            }
            Some(mut current) => { // here inside Box<Node> is mutable but without as_mut it immutable

                loop {

                    match current.next {
                        None => {
                            break;
                        }
                        Some(ref mut node) => {
                            current = node;
                        }
                    }

                }

                current.next = Some(new_node);
            }
        }

    }

    fn print_list(&self) {
        let mut header: &Option<Box<Node>> = &self.head;

        loop {
            match header {
                None => {
                    break;
                }
                Some(h) => {
                    print!("{} -> ", h.value);
                    header = &h.next;
                }
            }
        }

        println!("FIN");
    }

}

fn main() {
    let mut list: List = List::new();

    loop {
        print!("Enter the value... ");
        io::stdout().flush().expect("The new_line cannot be flushed");

        let mut str_num: String = String::new();
        io::stdin().read_line(&mut str_num).expect("Cannot able to read");

        let num: i32 = match str_num.trim().parse() {
            Ok(n) => n,
            Err(_) => -1,
        };

        if num == -1 {
            break;
        }

        list.append(num);
    }

    list.print_list();
}
