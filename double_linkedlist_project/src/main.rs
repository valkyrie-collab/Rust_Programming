use std::rc::Rc;
use std::cell::RefCell;

struct Node {
    value: i32,
    next: Option<Rc<RefCell<Node>>>,
    prev: Option<Rc<RefCell<Node>>>
}

struct DoubleList {
    head: Option<Rc<RefCell<Node>>>,
    tail: Option<Rc<RefCell<Node>>>
}

impl Node {

    fn new(value: i32) -> Node {
        Node {
            value: value,
            next: None,
            prev: None
        }
    }

}

impl DoubleList {

    fn new() -> DoubleList {
        DoubleList { head: None, tail: None }
    }

    fn append(&mut self, value: i32) {
        let new_node: Rc<RefCell<Node>> = Rc::new(RefCell::new(Node::new(value)));

        match self.head.clone() {
            None => {
                self.head = Some(new_node.clone());
                self.tail = Some(new_node);
            }
            Some(mut current) => {                
                
                loop {
                    let next = current.borrow().next.clone();

                    match next {
                        None => {
                            break;
                        }
                        Some(c) => {
                            current = c
                        }
                    }

                }

                current.borrow_mut().next = Some(new_node.clone());
                new_node.borrow_mut().prev = Some(current);

                self.tail = Some(new_node);
            }
            
        }

    }

    fn print_list_head(&self) {
        let mut header: Option<Rc<RefCell<Node>>> = self.head.clone();

        loop {

            match header {
                None => {
                    break;
                }
                Some(node) => {
                    print!("{} -> ", node.borrow().value);
                    header = node.borrow_mut().next.clone();
                }
            }

        }

        println!();
    }

    fn print_list_tail(&self) {
        let mut tailer: Option<Rc<RefCell<Node>>> = self.tail.clone();

        loop {

            match tailer {
                None => {
                    break;
                }
                Some(node) => {
                    print!("{} -> ", node.borrow().value);
                    tailer = node.borrow_mut().prev.clone();
                }
            }

        }

        println!();
    }
}

fn main() {
    let mut double_list: DoubleList = DoubleList::new();

    double_list.append(10);
    double_list.append(20);
    double_list.append(30);

    double_list.print_list_head();
    double_list.print_list_tail();
}
