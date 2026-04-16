use crate::node::single::Node;

pub struct Single<T> {
    head: Option<Box<Node<T>>>
}

impl<T: std::fmt::Display> Single<T> {
    pub fn append(&mut self, val: T) {
        let node: Box<Node<T>> = Box::new(Node::new(val));

        match self.head.as_mut() {
            None => self.head = Some(node),
            Some(mut current) => {

                loop {

                    match current.next {
                        None => break,
                        Some(ref mut node) => current = node
                    } 

                }

                current.next = Some(node);
            }
        }

    }

    pub fn show_list(&self) {
        let mut temp: Option<&Box<Node<T>>> = self.head.as_ref();

        while let Some(node) = temp {
            print!("{} -> ", node.value);
            temp = node.next.as_ref();
        }
        
    }
}