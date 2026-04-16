pub struct Node<'a> {
    pub value: &'a String,
    pub next: Option<Box<Node<'a>>>
}

impl<'a> Node<'a> {
    pub fn new(value: &'a String) -> Self {
        Node {
            value: value,
            next: None
        }
    }
}