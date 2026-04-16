use crate::tree::node::Node;

// static MAX: Mutex<u32> = Mutex::new(0);

pub struct Binary<'a> {
    root: Option<Box<Node<'a>>>
}

impl<'a> Binary<'a> {
    pub fn new() -> Self {
        Binary { root : None }
    }

    fn insert(tree: Option<Box<Node<'a>>>, brn: Box<Node<'a>>) -> Option<Box<Node<'a>>> {
        
        match tree {
            None => return Some(brn),
            Some(mut tree_brn) => {

                if tree_brn.tag >= brn.tag {
                    tree_brn.left = Self::insert(tree_brn.left, brn);
                } else {
                    tree_brn.right = Self::insert(tree_brn.right, brn);
                }

                Some(tree_brn)
            }
        }

    }

    pub fn add_node(&mut self, brn: Box<Node<'a>>) {
        self.root = Self::insert(self.root.take(), brn);
    }

    fn show(tree: Option<&Box<Node>>) {

        match tree {
            None => {return;}
            Some(brn) => {
                Self::show(brn.left.as_ref());
                print!("[ {} : {} ] -> ", brn.value, brn.tag);
                Self::show(brn.right.as_ref());
            }
        }
    }

    pub fn show_list(&self) {
        Self::show(self.root.as_ref());
        println!("FIN\n");
    }

    fn depth(tree: Option<&Box<Node>>, mut max: u32, mut temp: u32) -> u32 {

        match tree {
            None => {

                if max < temp {
                    max = temp;
                }

                return max; 
            }
            Some(node) => {
                temp += 1;
                max = Self::depth(node.left.as_ref(), max, temp);
                max = Self::depth(node.right.as_ref(), max, temp);
            }
        }

        max
    }

    pub fn find_depth(&self) -> u32 {
        // let max: MutexGuard<'_, u32> = MAX.lock().unwrap_or_else(|err| {
        //     println!("{:?}", err);
        //     process::exit(1);
        // });
        let max: u32 = Self::depth(self.root.as_ref(), 0, 0);

        max
    }
}