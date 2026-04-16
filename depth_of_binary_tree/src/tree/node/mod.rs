pub struct Node<'a> {
    pub value: &'a String,
    pub tag: i32,
    pub height: i32,
    pub left: Option<Box<Node<'a>>>,
    pub right: Option<Box<Node<'a>>>
}

impl<'a> Node<'a> {
    pub fn new(value: &'a String, tag: i32) -> Self {
        Node {
            value: value,
            tag: tag,
            height: 1,
            left: None,
            right: None
        }
    }
}