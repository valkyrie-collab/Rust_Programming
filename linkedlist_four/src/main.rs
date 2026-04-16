use std::rc::{Rc, Weak};
use std::cell::RefCell;

struct Node {
    value: i32,
    next: Option<Rc<RefCell<Node>>>,
    prev: Option<Weak<RefCell<Node>>>
}

struct List {
    head: Option<Rc<RefCell<Node>>>,
    tail: Option<Weak<RefCell<Node>>>
}

impl Node {
    fn new(value: i32) -> Self {
        Node {
            value: value,
            next: None,
            prev: None
        }
    }
}

impl List {
    fn new() -> Self {
        List { head: None, tail: None } 
    }

    fn append(&mut self, value: i32) {
        let new_node: Rc<RefCell<Node>> = Rc::new(RefCell::new(Node::new(value)));

        match Option::clone(&self.head) {
            None => {
                self.tail = Some(Rc::downgrade(&new_node));
                self.head = Some(new_node);
            }
            Some(mut current) => {

                loop {
                    let temp: Option<Rc<RefCell<Node>>> = Option::clone(&current.borrow().next);

                    match temp {
                        None => break,
                        Some(node) => {
                            current = node;
                        }
                    }

                }

                self.tail = Some(Rc::downgrade(&new_node));
                new_node.borrow_mut().prev = Some(Rc::downgrade(&current));
                current.borrow_mut().next = Some(new_node);
            }
        }

    }

    fn show_list(&self) {
        let mut header: Option<Rc<RefCell<Node>>> = Option::clone(&self.head);
        let mut tailer: Option<Weak<RefCell<Node>>> = Option::clone(&self.tail);

        while let Some(current) = header {
            print!("{} -> ", current.borrow().value);
            header = Option::clone(&current.borrow().next);
        }

        println!("FIN");
        print!("FIN");

        while let Some(current) = tailer {
            let val: Option<Rc<RefCell<Node>>> = current.upgrade();

            if let Some(node) = val {
                print!(" <- {}", node.borrow().value);
                tailer = Option::clone(&node.borrow().prev);
            } else {
                break;
            }

        }

        println!();
    }

}

fn main() {
    let dym_arr: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let mut list: List = List::new();

    for i in &dym_arr {
        list.append(*i);
    }

    list.show_list();
}
