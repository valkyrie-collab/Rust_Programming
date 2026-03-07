struct Root {
    value: i32,
    left: Option<Box<Root>>,
    right: Option<Box<Root>>
}

impl Root {

    fn new(value: i32) -> Self {
        Root {
            value: value,
            left: None,
            right: None
        }
    }

    fn binary_tree(root: Option<Box<Root>>, value: i32) -> Option<Box<Root>> {

        match root {
            None => {
                return Some(Box::new(Self::new(value)));
            }
            Some(mut n) => {

                if value < n.value {
                    n.left = Self::binary_tree(n.left, value);
                } else if value > n.value {
                    n.right = Self::binary_tree(n.right, value);
                }

                Some(n)
            }
        }

    }

    fn show_binary_tree(root: Option<Box<Root>>) {

        match root {
            None => {
                return;
            }
            Some(node) => {
                Self::show_binary_tree(node.left);
                print!("{} -> ", node.value);
                Self::show_binary_tree(node.right);
            }
        }

    }
}

fn main() {
    let mut root: Option<Box<Root>> = None;
    let arr: [i32; 5] = [5, 2, 4, 1, 6];

    for i in arr {
        root = Root::binary_tree(root, i);
    }

    Root::show_binary_tree(root);
    println!("FIN");
}
