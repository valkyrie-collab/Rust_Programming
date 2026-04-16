// use std::fmt::Display;
// use std::cmp::Ord;
use crate::tree::node::Node;

pub struct Avl<'a> {
    root: Option<Box<Node<'a>>>
}

impl<'a> Avl<'a> {
    pub fn new() -> Self {
        Avl { root: None }
    }

    fn get_height(brn: Option<&Box<Node<'a>>>) -> i32 {

        match brn {
            None => 0,
            Some(node) => node.height
        }

    }

    fn get_balance(brn: Option<&Box<Node<'a>>>) -> i32 {

        match brn {
            None => 0,
            Some(node) => {
                Self::get_height(node.left.as_ref()) - Self::get_height(node.right.as_ref())
            }
        }

    }

    fn get_max(value_one: i32, value_two: i32) -> i32 {
        
        if value_one > value_two {
            return value_one;
        }

        value_two
    }

    fn right_rotate(mut head: Option<Box<Node<'a>>>) -> Option<Box<Node<'a>>> {
        let mut new_head: Option<Box<Node<'a>>> = match head.as_mut() {
            None => None,
            Some(node) => node.left.take()
        };

        let srt_node: Option<Box<Node<'a>>> = match new_head.as_mut() {
            None => None,
            Some(node) => node.right.take()
        };

        if let Some(n) = head.as_mut() {
            n.left = srt_node;
            n.height = 1 + Self::get_max(Self::get_height(n.left.as_ref()), Self::get_height(n.right.as_ref()));
        }

        if let Some(n) = new_head.as_mut() {
            n.right = head; 
            n.height = 1 + Self::get_max(Self::get_height(n.left.as_ref()), Self::get_height(n.right.as_ref()));
        }

        new_head
    }

    fn left_rotate(mut head: Option<Box<Node<'a>>>) -> Option<Box<Node<'a>>> {
        let mut new_head: Option<Box<Node<'a>>> = match head.as_mut() {
            None => None,
            Some(node) => node.right.take()
        };

        let srt_node: Option<Box<Node<'a>>> = match new_head.as_mut() {
            None => None,
            Some(node) => node.left.take()
        };

        if let Some(n) = head.as_mut() {
            n.right = srt_node;
            n.height = 1 + Self::get_max(Self::get_height(n.left.as_ref()), Self::get_height(n.right.as_ref()));
        }

        if let Some(n) = new_head.as_mut() {
            n.left = head; 
            n.height = 1 + Self::get_max(Self::get_height(n.left.as_ref()), Self::get_height(n.right.as_ref()));
        }

        new_head
    }

    fn insert(tree: Option<Box<Node<'a>>>, brn: Box<Node<'a>>) -> Option<Box<Node<'a>>> {

        match tree {
            None => {return Some(brn);}
            Some(mut node) => {
                let mut old_brn: Option<&Box<Node>> = None;
                
                if node.tag > brn.tag {
                    node.left = Self::insert(node.left, brn);
                    old_brn = node.left.as_ref();
                } else if node.tag < brn.tag {
                    node.right = Self::insert(node.right, brn);
                    old_brn = node.right.as_ref();
                } else {

                    if node.value > brn.value {
                        node.left = Self::insert(node.left, brn);
                        old_brn = node.left.as_ref();
                    } else if node.value < brn.value {
                        node.right = Self::insert(node.right, brn);
                        old_brn = node.right.as_ref();
                    }

                }

                node.height = 1 + Self::get_max(Self::get_height(node.left.as_ref()), Self::get_height(node.right.as_ref()));
                let balance: i32 = Self::get_balance(Some(&node));

                let brn_val: i32 = match old_brn {
                    None => 0,
                    Some(node) => node.tag
                }; 

                let val_left: i32 = match node.left.as_ref() {
                    None => 0,
                    Some(n) => n.tag
                };

                let val_right: i32 = match node.right.as_ref() {
                    None => 0,
                    Some(n) => n.tag
                };

                if balance > 1 && brn_val < val_left {
                    return Self::right_rotate(Some(node));
                }

                if balance < -1 && brn_val > val_right {
                    return Self::left_rotate(Some(node));
                }

                if balance > 1 && brn_val > val_left {
                    node.left = Self::left_rotate(node.left);
                    return Self::right_rotate(Some(node));
                }

                if balance < -1 && brn_val < val_right {
                    node.right = Self::right_rotate(node.right);
                    return Self::left_rotate(Some(node));
                }

                Some(node)
            }
        }

    }

    pub fn new_node(&mut self, brn: Box<Node<'a>>) {
        self.root = Self::insert(self.root.take(), brn);
    }

    fn show(tree: Option<&Box<Node<'a>>>) {

        match tree {
            None => {return;}
            Some(brn) => {
                Self::show(brn.left.as_ref());
                print!("[ {} : {} ] -> ", brn.value, brn.tag);
                Self::show(brn.right.as_ref());
            }
        }

    }

    pub fn show_tree(&self) {
        Self::show(self.root.as_ref());
        println!("FIN\n");
    }
}