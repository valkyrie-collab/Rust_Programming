struct Node {
    value: i32,
    next: Option<Box<Node>>,
}

struct LinkedList {
    head: Option<Box<Node>>,
}

impl Node {

    fn new(value: i32) -> Node {
        Node {
            value: value,
            next: None
        }
    }

}

impl LinkedList {

    fn new() -> Self {
        LinkedList { head: None }
    }

    fn append(&mut self, value: i32) {
        let new_node: Box<Node> = Box::new(Node::new(value));

        match self.head.as_mut() {
            None => {
                self.head = Some(new_node);
            } 
            Some(mut current) => {
                
                loop {
                    
                    match current.next {
                        None => {
                            break;
                        } 
                        Some(ref mut next) => {
                            current = next;
                        }
                    }

                }

                current.next = Some(new_node);
            }
        }
        
    }

    fn print(&self) {

        let mut current = &self.head;

        loop {

            match current {
                None => {
                    break;
                }
                Some(next) => {
                    print!("{} -> ", next.value);
                    current = &next.next;
                }
            }

        }

        println!("FIN");
    }

}

fn main() {

    let mut list = LinkedList::new();

    list.append(10);
    list.append(20);
    list.append(30);
    list.append(40);

    list.print();
}