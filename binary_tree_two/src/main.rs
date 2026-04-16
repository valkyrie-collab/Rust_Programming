struct Node {
    value: i32,
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
            left: None,
            right: None
        }
    }

}

impl Tree {

    fn new() -> Self {
        Tree { root: None }
    }

    fn push(node: Option<Box<Node>>, value: i32) -> Option<Box<Node>> {

        match node {
            None => {
                return Some(Box::new(Node::new(value)));
            }
            Some(mut n) => {
                
                if value < n.value {
                    n.left = Self::push(n.left, value);
                } else if value > n.value {
                    n.right = Self::push(n.right, value);
                }

                Some(n)
            }
        }

    }

    fn binary_tree(&mut self, value: i32) {
        self.root = Self::push(self.root.take(), value);
    }

    fn show(node: Option<Box<Node>>) {

        match node {
            None => {
                return;
            }
            Some(n) => {
                Self::show(n.left);
                print!("{} -> ", n.value);
                Self::show(n.right);
            }
        }

    }

    fn show_binary_tree(self) {
        Self::show(self.root);
    }
}

fn main() {
    let mut tree: Tree = Tree::new();
    let arr: [i32; 5] = [5, 4, 1, 7, 8];

    for i in arr {
        tree.binary_tree(i);
    }

    tree.show_binary_tree();
    println!("FIN");
}
