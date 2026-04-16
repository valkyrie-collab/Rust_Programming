use std::io::{self, Write};
use std::rc::Rc;
use std::cell::RefCell;

struct Node {
    value: i32,
    next: Option<Rc<RefCell<Node>>>,
    prev: Option<Rc<RefCell<Node>>>
}

struct List {
    head: Option<Rc<RefCell<Node>>>,
    tail: Option<Rc<RefCell<Node>>>
}

impl Node {

    fn new(value: i32) -> Node {
        Node {
            value: value,
            next: None,
            prev: None
        }
    }

}

impl List {

    fn new () -> List {
        List {
            head: None,
            tail: None
        }
    }

    fn append(&mut self, value: i32) {
        let new_node: Rc<RefCell<Node>> = Rc::new(RefCell::new(Node::new(value)));

        match self.head.clone() {
            None => {
                self.head = Some(new_node.clone());
                self.tail = Some(new_node);
            }
            Some(mut current) => {

                loop {
                    let temp: Option<Rc<RefCell<Node>>> = current.borrow().next.clone();

                    match temp {
                        None => {
                            break;
                        }
                        Some(node) => {
                            current = node;
                        }
                    }

                }

                current.borrow_mut().next = Some(new_node.clone());
                new_node.borrow_mut().prev = Some(current);
                self.tail = Some(new_node);
            }

        }

    }

    fn print_head(&self) {
        let mut header: Option<Rc<RefCell<Node>>> = self.head.clone();

        loop {

            match header {
                None => {
                    break;
                }
                Some(node) => {
                    print!("{} -> ", node.borrow().value);
                    header = node.borrow().next.clone();
                }
            }

        }

        println!("FIN");
    }

    fn print_tail(&self) {
        let mut tailer: Option<Rc<RefCell<Node>>> = self.tail.clone();
        print!("FIN");

        loop {

            match tailer {
                None => {
                    break;
                }
                Some(node) => {
                    print!(" <- {}", node.borrow().value);
                    tailer = node.borrow().prev.clone();
                }
            }

        }

        println!();
    }

}

fn main() {
    let mut list: List = List::new();

    loop {
        print!("Enter the number: ");
        io::stdout().flush().expect("Cannot flush new line");

        let mut str_num: String = String::new();
        io::stdin().read_line(&mut str_num).expect("Cannot read line");

        let num: i32 = match str_num.trim().parse() {
            Ok(n) => n,
            Err(_) => -1,
        };

        if num == -1 {
            break;
        }

        list.append(num);
    }

    list.print_head();
    list.print_tail();
}
