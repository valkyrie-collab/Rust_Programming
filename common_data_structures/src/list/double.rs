use crate::node::double::Node;
use std::cell::RefCell;
use std::rc::{Rc, Weak};

pub struct Double<T> {
    head: Option<Rc<RefCell<Node<T>>>>,
    tail: Option<Weak<RefCell<Node<T>>>>
}

impl<T: std::fmt::Display> Double<T> {
    pub fn append(&mut self, val: T) {
        let node: Rc<RefCell<Node<T>>> = Rc::new(RefCell::new(Node::new(val)));

        match Option::clone(&self.head) {
            None => {
                self.head = Some(Rc::clone(&node));
                self.tail = Some(Rc::downgrade(&node));
            }
            Some(mut current) => {
                let mut temp: Option<Rc<RefCell<Node<T>>>>;

                loop {
                    temp = Some(Rc::clone(current.borrow_mut().next.as_ref().unwrap()));
                    
                    match temp {
                        None => break,
                        Some(node) => {
                            current = node;
                        }
                    }

                }

                self.tail = Some(Rc::downgrade(&node));
                node.borrow_mut().prev = Some(Rc::downgrade(&node));
                current.borrow_mut().next = Some(node);
            }
        }

    }

    pub fn show_list(&self) {
        let mut temp_head: Option<Rc<RefCell<Node<T>>>> = Option::clone(&self.head);
        let mut temp_tail: Option<Rc<RefCell<Node<T>>>> = Weak::upgrade(self.tail.as_ref().unwrap());

        while let Some(node) = temp_head {
            print!("{} -> ", node.borrow().value);
            temp_head = Some(Rc::clone(node.borrow().next.as_ref().unwrap()));
        }

        while let Some(node) = temp_tail {
            print!("{} -> ", node.borrow().value);
            temp_tail = node.borrow().prev.as_ref().unwrap().upgrade()
        }
        
    }
}