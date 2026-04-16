use std::rc::{Rc, Weak};
use std::cell::RefCell;

pub struct Node {
    pub value: String,
    pub next: Option<Rc<RefCell<Node>>>,
    pub prev: Option<Weak<RefCell<Node>>>
}

impl Node {
    pub fn new(node_value: String) -> Self {
        Node {value: node_value, next: None, prev: None}
    }
}