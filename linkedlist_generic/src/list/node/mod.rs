use std::{rc::Rc, cell::RefCell};

pub struct Node<T> {
    pub value: T,
    pub next: Option<Box<Node<T>>>,
}

pub struct DNode<T> {
    pub value: T,
    pub next: Option<Rc<RefCell<DNode<T>>>>,
    pub prev: Option<Rc<RefCell<DNode<T>>>>
}

impl<T> Node<T> {
    pub fn new(value: T) -> Self {
        Node {
            value: value,
            next: None
        }
    }
}

impl<T> DNode<T> {
    pub fn new(value: T) -> Self {
        DNode {
            value: value,
            next: None,
            prev: None
        }
    }
}