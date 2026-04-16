use std::rc::Rc;
use std::cell::RefCell;

pub struct Node {
    pub value: i32,
    pub next: Option<Rc<RefCell<Node>>>,
    pub prev: Option<Rc<RefCell<Node>>>
}

impl Node {
    pub fn new(value: i32) -> Self {
        Node {
            value,
            next: None,
            prev: None
        }
    }
}