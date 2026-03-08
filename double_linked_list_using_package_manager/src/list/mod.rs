mod node;
use node::Node;
use std::{cell::RefCell, rc::Rc};

pub struct List {
    head: Option<Rc<RefCell<Node>>>,
    tail: Option<Rc<RefCell<Node>>>
}

impl List {
    pub fn new() -> Self {
        List { head: None, tail: None }
    }

    pub fn insert(&mut self, value: i32) {
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

    pub fn show_list(self) {
        let mut header: Option<Rc<RefCell<Node>>> = self.head.clone();
        let mut tailer: Option<Rc<RefCell<Node>>> = self.tail.clone();

        println!("The list from first to last");

        loop {

            match header {
                None => break,
                Some(current) => {
                    print!("{} -> ", current.borrow().value);
                    header = current.borrow().next.clone();
                }
            }

        }

        println!("FIN");
        println!("The list from last to first");
        print!("FIN");

        loop {

            match tailer {
                None => break,
                Some(current) => {
                    print!(" <- {}", current.borrow().value);
                    tailer = current.borrow().prev.clone();
                }
            }

        }

        println!()
    }
}