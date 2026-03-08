mod node;
use node::Node;

pub struct List {
    head: Option<Box<Node>>
}

impl List {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn insert(&mut self, value: i32) {
        let new_value: Box<Node> = Box::new(Node::new(value));

        match self.head.as_mut() {
            None => {
                self.head = Some(new_value)
            }
            Some(mut node) => {

                loop {
                    
                    match node.next {
                        None => break,
                        Some(ref mut n) => {
                            node = n;
                        }
                    }

                }
                
                node.next = Some(new_value);
            }
        }
        
    }

    pub fn show_list(self) {
        let mut header: Option<Box<Node>> = self.head;

        loop {
            
            match header {
                None => break,
                Some(current) => {
                    print!("{} -> ", current.value);
                    header = current.next;
                }
            }

        }

        println!("FIN");
    }
}