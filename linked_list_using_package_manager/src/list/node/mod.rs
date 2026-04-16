pub struct Node {
    pub value: i32,
    pub next: Option<Box<Node>>
}

impl Node {
    pub fn new(value: i32) -> Self {
        Node {
            value: value,
            next: None
        }
    }
}