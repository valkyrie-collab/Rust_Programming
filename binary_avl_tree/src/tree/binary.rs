use crate::tree::node::Node;
use std::fmt::Display;
use std::cmp::Ordering;

pub struct Binary<T> {
    root: Option<Box<Node<T>>>
}

impl<T:Display + Ord> Binary<T> {
    pub fn new() -> Self {
        Binary { root: None }
    }

    fn insert(tree: Option<Box<Node<T>>>, brn: Box<Node<T>>) -> Option<Box<Node<T>>> {

        if let None = tree {
            return Some(brn);
        }

        if let Some(mut tree_brn) = tree {

            match tree_brn.value.cmp(&brn.value) {
                Ordering::Greater => {
                    tree_brn.right = Self::insert(tree_brn.right, brn);
                }
                Ordering::Less => {
                    tree_brn.left = Self::insert(tree_brn.left, brn);
                }
                Ordering::Equal => {
                    
                    if tree_brn.count > brn.count {
                        tree_brn.right = Self::insert(tree_brn.right, brn);
                    } else if tree_brn.count < brn.count {
                        tree_brn.left = Self::insert(tree_brn.left, brn);
                    }

                }
            }

            return Some(tree_brn);
        }

        None
    }

    pub fn append(&mut self, brn: Box<Node<T>>) {
        self.root = Self::insert(self.root.take(), brn);
    }

    fn show(tree: Option<&Box<Node<T>>>) {

        match tree {
            None => {
                return;
            }
            Some(current) => {
                Self::show(current.left.as_ref());
                print!("{} -> ", current.value);
                Self::show(current.right.as_ref());
            }
        }

    }

    pub fn show_tree(&self) {
        Self::show(self.root.as_ref());
        println!("FIN");
    }
}