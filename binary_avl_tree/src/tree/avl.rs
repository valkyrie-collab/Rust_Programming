use crate::tree::node::Node;
use std::cmp::Ordering;

pub struct Avl<T> {
    root: Option<Box<Node<T>>>
}

impl<T: std::fmt::Display + Ord> Avl<T> {
    pub fn new() -> Self {
        Avl { root : None }
    }

    fn get_height(node: Option<&Box<Node<T>>>) -> i32 {

        match node {
            None => 0,
            Some(n) => n.height
        }

    }

    fn get_balance(node: Option<&Box<Node<T>>>) -> i32 {

        match node {
            None => 0,
            Some(n) => {
                Self::get_height(n.left.as_ref()) - Self::get_height(n.right.as_ref())
            }
        }

    }

    fn get_max(val_one: i32, val_two: i32) -> i32 {

        if val_one > val_two {
            return val_one;
        }

        val_two
    }

    fn right_rotate(mut head: Option<Box<Node<T>>>) -> Option<Box<Node<T>>> {
        let mut new_head: Option<Box<Node<T>>> = match head.as_mut() {
            None => None,
            Some(node) => node.left.take()
        };

        let srt_node: Option<Box<Node<T>>> = match new_head.as_mut() {
            None => None,
            Some(node) => node.right.take()
        };

        match head.as_mut() {
            None => {},
            Some(node) => {
                node.left = srt_node;
                node.height = 1 + Self::get_max(
                    Self::get_height(node.left.as_ref()), Self::get_height(node.right.as_ref())
                );
            }
        }

        match new_head.as_mut() {
            None => {},
            Some(node) => {
                node.right = head;
                node.height = 1 + Self::get_max(
                    Self::get_height(node.left.as_ref()), Self::get_height(node.right.as_ref())
                ); 
            }
        }

        new_head
    }

    fn left_rotate(mut head: Option<Box<Node<T>>>) -> Option<Box<Node<T>>> {
        let mut new_head: Option<Box<Node<T>>> = match head.as_mut() {
            None => None,
            Some(n) => n.right.take()
        };
        let srt_node: Option<Box<Node<T>>> = match new_head.as_mut() {
            None => None,
            Some(n) => n.left.take()
        };

        match head.as_mut() {
            None => {},
            Some(n) => {
                n.right = srt_node;
                n.height = 1 + Self::get_max(
                    Self::get_height(n.left.as_ref()), Self::get_height(n.right.as_ref())
                );
            }
        }

        match new_head.as_mut() {
            None => {},
            Some(n) => {
                n.left = head;
                n.height = 1 + Self::get_max(
                    Self::get_height(n.left.as_ref()), Self::get_height(n.right.as_ref())
                );
            }
        }

        None
    }

    fn insert(tree: Option<Box<Node<T>>>, brn: Box<Node<T>>) -> Option<Box<Node<T>>> {

        match tree {
            None => Some(brn),
            Some(mut current) => {
                let old_brn: Option<&Box<Node<T>>>;

                if current.count > brn.count {
                    current.left = Self::insert(current.left, brn);
                    old_brn = current.left.as_ref();
                } else if current.count < brn.count {
                    current.right = Self::insert(current.right, brn);
                    old_brn = current.right.as_ref();
                } else {

                    if current.value > brn.value {
                        current.left = Self::insert(current.left, brn);
                        old_brn = current.left.as_ref();
                    } else {
                        current.right = Self::insert(current.right, brn);
                        old_brn = current.right.as_ref();
                    }

                }

                current.height = 1 + Self::get_max(
                    Self::get_height(current.left.as_ref()), Self::get_height(current.right.as_ref())
                );
                let balance: i32 = Self::get_balance(Some(&current));

                let brn_left_val: u32 = match old_brn {
                    None => 0,
                    Some(n) => n.count
                };
                let brn_right_val: u32 = match old_brn {
                    None => 0,
                    Some(n) => n.count
                };

                let left_val: u32 = match current.left.as_ref() {
                    None => 0,
                    Some(n) => n.count
                };
                let right_val: u32 = match current.right.as_ref() {
                    None => 0,
                    Some(n) => n.count
                };

                if balance > 1 && brn_left_val < left_val {
                    return Self::right_rotate(Some(current));
                }

                if balance < -1 && brn_right_val > right_val {
                    return Self::left_rotate(Some(current));
                }

                if balance > 1 && brn_left_val > left_val {
                    current.left = Self::left_rotate(current.left);
                    return Self::right_rotate(Some(current));
                }

                if balance < -1 && brn_right_val < right_val {
                    current.right = Self::right_rotate(current.right);
                    return Self::left_rotate(Some(current));
                }

                Some(current)
            }
        }
 
    }

    pub fn append(&mut self, brn: Box<Node<T>>) {
        self.root = Self::insert(self.root.take(), brn);
    }

    fn show(tree: Option<&Box<Node<T>>>) {

        match tree {
            None => return,
            Some(node) => {
                Self::show(node.left.as_ref());
                print!("{} -> ", node.value);
                Self::show(node.right.as_ref());
            }
        }

    }

    pub fn show_tree(&self) {
        Self::show(self.root.as_ref());
    }
}