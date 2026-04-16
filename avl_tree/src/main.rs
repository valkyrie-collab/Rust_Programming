struct Node {
    value: i32,
    height: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>
}

struct Tree {
    root: Option<Box<Node>>
}

impl Node {
    fn new(value: i32) -> Self {
        Node {
            value: value,
            height: 1,
            left: None,
            right: None 
        }
    }
}

impl Tree {
    fn new() -> Self {
        Tree { root : None }
    }

    fn get_height(node: &Option<Box<Node>>) -> i32 {

        match node {
            None => {
                return 0;
            }
            Some(n) => {
                n.height
            }
        }

    }

    fn get_balance(node: Option<&Box<Node>>) -> i32 {

        match node {
            None => {
                return 0;
            }
            Some(n) => {
                Self::get_height(&n.left) - Self::get_height(&n.right)
            }
        }

    }

    fn max(val_one: i32, val_two: i32) -> i32 {
        
        if val_one > val_two {
            val_one
        } else {
            val_two
        }

    }

    fn right_rotate(mut head: Option<Box<Node>>) -> Option<Box<Node>> {
        let mut head_left: Option<Box<Node>> = match head.as_mut() {
            None => None,
            Some(n) => n.left.take() 
        };
        let srt_nodes: Option<Box<Node>> = match head_left.as_mut() {
            None => None,
            Some(n) => n.right.take()
        }; 

        match head.as_mut() {
            None => {},
            Some(h) => {
                h.left = srt_nodes;
                h.height = 1 + Self::max(Self::get_height(&h.left), Self::get_height(&h.right));
            }
        };

        match head_left.as_mut() {
            None => {},
            Some(h) => {
                h.right = head;
                h.height = 1 + Self::max(Self::get_height(&h.left), Self::get_height(&h.right));
            }
        };

        head_left
    }

    fn left_rotate(mut head: Option<Box<Node>>) -> Option<Box<Node>> {
        let mut head_right: Option<Box<Node>> = match head.as_mut() {
            None => None,
            Some(h) => h.right.take()
        };
        let srt_nodes: Option<Box<Node>> = match head_right.as_mut() {
            None => None,
            Some(h) => h.left.take()
        };

        match &mut head {
            None => {}
            Some(h) => {
                h.right = srt_nodes;
                h.height = 1 + Self::max(Self::get_height(&h.left), Self::get_height(&h.right));
            }
        }

        match &mut head_right {
            None => {},
            Some(h) => {
                h.left = head;
                h.height = 1 + Self::max(Self::get_height(&h.left), Self::get_height(&h.right));
            }
        }
        
        head_right
    }

    fn insert(node: Option<Box<Node>>, value: i32) -> Option<Box<Node>> {

        match node {
            None => {
                return Some(Box::new(Node::new(value)));
            }
            Some(mut n) => {

                if value < n.value {
                    n.left = Self::insert(n.left, value);
                } else if value > n.value {
                    n.right = Self::insert(n.right, value);
                } else {
                    return Some(n);
                }

                n.height = 1 + Self::max(Self::get_height(&n.left), Self:: get_height(&n.right));
                let bal: i32 = Self::get_balance(Some(&n));
                let left_value: i32 = match n.left.as_ref() {
                    None => {
                        -1
                    }
                    Some(n) => {
                        n.value
                    }
                };
                let right_value: i32 = match n.right.as_ref() {
                    None => {
                        -1
                    }
                    Some(n) => {
                        n.value
                    }
                };

                if bal > 1 && value < left_value {
                    return Self::right_rotate(Some(n));
                }

                if bal < -1 && value > right_value {
                    return Self::left_rotate(Some(n));
                }

                if bal > 1 && value > left_value {
                    n.left = Self::left_rotate(n.left);
                    return Self::right_rotate(Some(n));
                }

                if bal < -1 && value < right_value {
                    n.right = Self::right_rotate(n.right);
                    return Self::left_rotate(Some(n));
                }

                Some(n)
            }
        }
        
    }

    fn binary_tree(&mut self, value: i32) {
        self.root = Self::insert(self.root.take(), value);
    }

    fn show(root: Option<Box<Node>>) {

        match root {
            None => return,
            Some(r) => {
                Self::show(r.left);
                print!("{} -> ", r.value);
                Self::show(r.right);
            }
        }
    }

    fn show_avl_tree(self) {
        Self::show(self.root);
    }
}

fn main() {
    let arr: [i32; 5] = [5, 4, 7, 9, 3];

    let mut tree: Tree = Tree::new();

    for i in arr {
        tree.binary_tree(i);
    }
    
    tree.show_avl_tree();
    println!("FIN");
}
