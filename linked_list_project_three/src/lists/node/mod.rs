use std::{rc::Rc, cell::RefCell};

pub struct Node {
    pub value: i32,
    pub next: Option<Box<Node>>
}

pub struct DNode {
    pub value: i32,
    pub next: Option<Rc<RefCell<DNode>>>,
    pub prev: Option<Rc<RefCell<DNode>>>
}

impl Node {
    pub fn new(value: i32) -> Self {
        Node {
            value: value,
            next: None
        }
    }
}

impl DNode {
    pub fn new(value: i32) -> Self {
        DNode {
            value: value,
            next: None,
            prev: None
        }
    }
}