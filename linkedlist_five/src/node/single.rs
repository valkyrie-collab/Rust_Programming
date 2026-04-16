pub struct Node {
    pub value: String,
    pub next: Option<Box<Node>> 
}

impl Node {
    pub fn new(node_value: String) -> Self {
        Node {value: node_value, next: None}
    }
}