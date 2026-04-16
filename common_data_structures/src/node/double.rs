use std::cell::RefCell;
use std::rc::{Rc, Weak};

pub struct Node<T> {
    pub value: T,
    pub next: Option<Rc<RefCell<Node<T>>>>,
    pub prev: Option<Weak<RefCell<Node<T>>>>
}

impl<T: std::fmt::Display> Node<T> {
    pub fn new(val: T) -> Self {
        Node {value: val, next: None, prev: None}
    }
}