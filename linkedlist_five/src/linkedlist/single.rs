use crate::node::single::Node;

pub struct Single {
    head: Option<Box<Node>>
}

impl Single {
    pub fn new() -> Self {
        Single {head: None}
    }

    pub fn append_list(&mut self, node_value: String) {
        let node: Box<Node> = Box::new(Node::new(node_value));

        match self.head.as_mut() {
            None => {
                self.head = Some(node)
            }
            Some(mut current) => {

                loop {

                    match current.next {
                        Some(ref mut node) => {
                            current = node;
                        }
                        None => {
                            break;
                        }
                    }

                }

                current.next = Some(node);
            }
        }

    }

    pub fn show_list(&self) {
        let mut temp: &Option<Box<Node>> = &self.head;

        while let Some(node) = temp {
            print!("{} -> ", node.value);

            temp = &node.next;
        }

        println!("FIN");
    }
}
