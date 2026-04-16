pub struct Node<T> {
    pub value: T,
    pub next: Option<Box<Node<T>>>
}

impl<T: std::fmt::Display> Node<T> {
    pub fn new(val: T) -> Self {
        Node {value: val, next: None}
    }
}