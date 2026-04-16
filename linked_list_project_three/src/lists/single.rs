use crate::lists::node::Node;
use crate::lists::traits::ListOp;

pub struct SingleList {
    head: Option<Box<Node>>
}

impl ListOp for SingleList {
    fn new() -> Self {
        SingleList { head: None }
    }

    fn append(&mut self, value: i32) {
        let new_node: Box<Node> = Box::new(Node::new(value));
        // let mut fr: &mut Box<Node> = (&mut Some(new_node)).unwrap();

        match self.head.as_mut() {
            None => {
                self.head = Some(new_node);
            },
            Some(mut current) => {

                loop {

                    match current.next {
                        None => break,
                        Some(ref mut node) => {
                            current = node;
                        }
                    }

                }

                current.next = Some(new_node);
            }

        }

    }

    fn show_list(&mut self) {
        let mut header: Option<&mut Box<Node>> = self.head.as_mut();

        loop {

            match header {
                None => break,
                Some(node) => {
                    print!("{} -> ", node.value);
                    header = node.next.as_mut();
                }
            }

        }

        println!("FIN")
    }
}