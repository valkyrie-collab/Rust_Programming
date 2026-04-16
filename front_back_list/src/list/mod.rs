mod node;

use node::Node;

pub struct List<'a> {
    head: Option<Box<Node<'a>>>
}

impl<'a> List<'a> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push_back(&mut self, value: &'a String) {
        let new_node: Box<Node<'a>> = Box::new(Node::new(value));

        match self.head.as_mut() {
            None => {
                self.head = Some(new_node);
            }
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

    pub fn push_front(&mut self, value: &'a String) {
        let old_node: Option<Box<Node<'a>>> = self.head.take();
        let mut new_node: Box<Node<'a>> = Box::new(Node::new(value));

        new_node.next = old_node;
        self.head = Some(new_node);
    }

    pub fn show(&self) {
        let mut header: Option<&Box<Node<'a>>> = self.head.as_ref();

        loop {

            match header {
                None => break,
                Some(node) => {
                    print!(" {} -> ", node.value);
                    header = node.next.as_ref();
                }
            }

        }
        
        println!();
    }
}