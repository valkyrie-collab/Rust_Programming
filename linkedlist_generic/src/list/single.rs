use crate::list::node::Node;

pub struct Single<T> {
    head: Option<Box<Node<T>>>
}

impl<T: std::fmt::Display> Single<T> {
    pub fn new() -> Self {
        Single { head: None }
    }

    pub fn append(&mut self, value: T) {
        let new_node: Box<Node<T>> = Box::new(Node::new(value));

        match self.head.as_mut() {
            None => {
                self.head = Some(new_node);
            }
            Some(mut current) => {

                loop {

                    match current.next {
                        None => break,
                        Some(ref mut node) => {
                            current = node
                        }
                    }

                }

                current.next = Some(new_node);
            }
        }

    }

    pub fn show_list(&self) {
        let mut header: Option<&Box<Node<T>>> = self.head.as_ref();

        loop {

            match header {
                None => break,
                Some(current) => {
                    print!("{} -> ", current.value);
                    header = current.next.as_ref();
                }
            }

        }

        println!("FIN");
    }

    pub fn retrive(self) -> Vec<T> {
        let mut header: Option<Box<Node<T>>> = self.head;
        let mut result: Vec<T> = Vec::new();

        loop {

            match header {
                None => break,
                Some(current) => {
                    result.push(current.value);
                    header = current.next;
                }
            }

        }

        result
    }
}