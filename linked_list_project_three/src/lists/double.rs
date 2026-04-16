use crate::lists::{node::DNode, traits::ListOp};
use std::{rc::Rc, rc::Weak, cell::RefCell};

pub struct DoubleList {
    head: Option<Rc<RefCell<DNode>>>,
    tail: Option<Rc<RefCell<DNode>>>
}

impl ListOp for DoubleList {
    fn new() -> Self {
        DoubleList { head: None, tail: None }
    }
    
    fn append(&mut self, value: i32) {
        let new_node: Rc<RefCell<DNode>> = Rc::new(RefCell::new(DNode::new(value)));

        match Option::clone(&self.head) {
            None => {
                self.head = Some(new_node.clone());
                self.tail = Some(new_node);
            }
            Some(mut current) => {

                loop {
                    let temp: Option<Rc<RefCell<DNode>>> = Option::clone(&current.borrow().next);
                    
                    match temp {
                        None => break,
                        Some(node) => {
                            current = node;
                        }
                    }

                }

                current.borrow_mut().next = Some(Rc::clone(&new_node));
                new_node.borrow_mut().prev = Some(Rc::downgrade(&current));
                self.tail = Some(new_node);
            }
        }

    }

    fn show_list(&mut self) {
        let mut header: Option<Rc<RefCell<DNode>>> = self.head.clone();
        let tailer: Option<Rc<RefCell<DNode>>> = self.tail.clone();

        while let Some(current) = header {
            print!("{} -> ", current.borrow().value);
            header = current.borrow().next.clone();
        }

        println!("FIN");
        print!("FIN");

        if let Some(t) = tailer {
            print!(" <- {}", t.borrow().value);
            let mut tailer: Option<Weak<RefCell<DNode>>> = Option::clone(&t.borrow().prev);

            while let Some(n) = tailer {
                
                if let Some(n2) = n.upgrade() {
                    print!(" <- {}", n2.borrow().value);
                    tailer = Option::clone(&n2.borrow().prev);
                } else {
                    break;
                }

            }

        }

        println!();
    }     
}


use crate::lists::{node::DNode, traits::ListOp};
use std::{rc::Rc, cell::RefCell};

pub struct DoubleList {
    head: Option<Rc<RefCell<DNode>>>,
    tail: Option<Rc<RefCell<DNode>>>
}

impl ListOp for DoubleList {
    fn new() -> Self {
        DoubleList { head: None, tail: None }
    }
    
    fn append(&mut self, value: i32) {
        let new_node: Rc<RefCell<DNode>> = Rc::new(RefCell::new(DNode::new(value)));

        match self.head.clone() {
            None => {
                self.head = Some(new_node.clone());
                self.tail = Some(new_node);
            }
            Some(mut current) => {

                loop {
                    let temp: Option<Rc<RefCell<DNode>>> = current.borrow().next.clone();
                    
                    match temp {
                        None => break,
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

    fn show_list(&mut self) {
        let mut header: Option<Rc<RefCell<DNode>>> = self.head.clone();
        let mut tailer: Option<Rc<RefCell<DNode>>> = self.tail.clone();

        while let Some(current) = header {
            print!("{} -> ", current.borrow().value);
            header = current.borrow().next.clone();
        }

        println!("FIN");
        print!("FIN");

        while let Some(current) = tailer {
            print!(" <- {}", current.borrow().value);
            tailer = current.borrow().prev.clone();
        }

        println!();
    }     
}

