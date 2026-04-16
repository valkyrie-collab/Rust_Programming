use crate::node::double::Node;
use std::rc::{Rc, Weak};
use std::cell::RefCell;

pub struct Double {
    head: Option<Rc<RefCell<Node>>>,
    tail: Option<Weak<RefCell<Node>>>
}

impl Double {
    pub fn new() -> Self {
        Double {head: None, tail: None}
    }

    pub fn append_next(&mut self, node_value: String) {
        let node: Rc<RefCell<Node>> = Rc::new(RefCell::new(Node::new(node_value)));

        match Option::clone(&self.head) {
            None => {
                self.tail = Some(Rc::downgrade(&node));
                self.head = Some(node);
            }
            Some(mut current) => {

                loop {
                    let temp: Option<Rc<RefCell<Node>>> = Option::clone(&current.borrow().next);

                    match temp {
                        None => {
                            break;
                        }
                        Some(node) => {
                            current = node;
                        }
                    }

                }

                self.tail = Some(Rc::downgrade(&node));
                node.borrow_mut().prev = Some(Rc::downgrade(&current));
                current.borrow_mut().next = Some(node);
            }
        }

    }

    pub fn show_list(&self) {
        let mut temp: Option<Rc<RefCell<Node>>> = Option::clone(&self.head);

        while let Some(node) = temp {
            print!("{} -> ", node.borrow().value);

            temp = Option::clone(&node.borrow().next);
        }

        print!("FIN\nFIN");

        let mut temp: Option<Rc<RefCell<Node>>> = Weak::upgrade(&Option::clone(&self.tail).unwrap());

        while let Some(node) = temp {
            print!(" <- {}", node.borrow().value);

            temp = if let Some(t) = Option::clone(&node.borrow().prev) {
                t.upgrade()
            } else { None }
        }
    }
}