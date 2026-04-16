use std::rc::Rc;
use std::cell::RefCell;
use crate::list::node::DNode;

pub struct Double<T> {
    head: Option<Rc<RefCell<DNode<T>>>>,
    tail: Option<Rc<RefCell<DNode<T>>>>
}

impl<T: std::fmt::Display> Double<T> {
    pub fn new() -> Self {
        Double { head: None, tail: None }
    }

    pub fn append(&mut self, value: T) {
        let new_node: Rc<RefCell<DNode<T>>> = Rc::new(RefCell::new(DNode::new(value)));

        match self.head.clone() {
            None => {
                self.head = Some(new_node.clone());
                self.tail = Some(new_node);
            }
            Some(mut current) => { 
                
                loop {
                    let temp: Option<Rc<RefCell<DNode<T>>>> = current.borrow().next.clone();
                    
                    match temp {
                        None => break,
                        Some(node) => {
                            current = node
                        }
                    }

                }

                current.borrow_mut().next = Some(new_node.clone());
                new_node.borrow_mut().prev = Some(current);
                self.tail = Some(new_node);
            }
        }

    }

    pub fn show_list(&self) {
        let mut header: Option<Rc<RefCell<DNode<T>>>> = self.head.clone();
        let mut tailer: Option<Rc<RefCell<DNode<T>>>> = self.tail.clone();
        
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
        print!("FIN");

        loop {

            match tailer {
                None => break,
                Some(current ) => {
                    print!("<- {}", current.borrow().value);
                    tailer = current.borrow().prev.clone();
                }
            }

        }
        
    }
}