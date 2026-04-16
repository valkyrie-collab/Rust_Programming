struct Node {
    value: i32,
    right: Option<Box<Node>>,
    left: Option<Box<Node>>
}

struct Tree {
    root: Option<Box<Node>>
}

impl Node {
    fn new(val: i32) -> Self {
        Node {
            value: val,
            right: None,
            left: None
        }
    }
}

impl Tree {
    pub fn new() -> Self {
        Tree { root: None }
    }

    fn insert(tree: Option<Box<Node>>, val: i32) -> Option<Box<Node>> {

        match tree {
            None => {
               Some(Box::new(Node::new(val)))
            }
            Some(mut node) => {

                if node.value > val {
                    node.left = Self::insert(node.left, val);
                } else if node.value < val {
                    node.right = Self::insert(node.right, val);
                }

                Some(node)
            }
        }

    }

    pub fn push(&mut self, val: i32) {
        self.root = Self::insert(self.root.take(), val);
    }

    fn show(tree: &Option<Box<Node>>) {

        match tree {
            None => {}
            Some(node) => {
                Self::show(&node.left);
                print!("{} -> ", node.value);
                Self::show(&node.right);
            }
        }

    }

    pub fn show_tree(&self) {
        Self::show(&self.root);
    }

    fn count(tree: &Option<Box<Node>>, mut cnt: u32) -> u32 {

        match tree {
            None => { cnt }
            Some(node) => {

                if let None = &node.left && let None = &node.right {
                    cnt += 1;
                }

                cnt = Self::count(&node.left, cnt);
                cnt = Self::count(&node.right, cnt);
                cnt
            }
        }

    }

    pub fn count_leaf(&self) -> u32 {
        Self::count(&self.root, 0)
    }
}

fn main() {
    let mut binary_tree: Tree = Tree::new();

    binary_tree.push(5);
    binary_tree.push(1);
    binary_tree.push(2);
    binary_tree.push(7);
    binary_tree.push(6);
    binary_tree.push(4);

    binary_tree.show_tree();

    let count: u32 = binary_tree.count_leaf();
    println!("count of leaf {}", count);
}
