pub struct Node<T> {
    pub value: T,
    pub count: u32,
    pub height: i32,
    pub left: Option<Box<Node<T>>>,
    pub right: Option<Box<Node<T>>>
}

impl<T> Node<T> {
    pub fn new(value: T) -> Self {
        Node {
            value: value,
            count: 1,
            height: 1,
            left: None,
            right: None
        }
    }
}

