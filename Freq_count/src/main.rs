use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::process;
use std::io::{ Read, Write };
use std::str::SplitWhitespace;
use std::env::{ self, Args };

struct Data {
    word: String,
    freq: u32,
    height: i32,
    right: Option<Box<Data>>,
    left: Option<Box<Data>>
}

struct Tree {
    root: Option<Box<Data>>
}

impl Data {
    fn new(wrd: String, f: u32) -> Self {
        Data {
            word: wrd,
            freq: f,
            height: 1,
            right: None,
            left: None
        }
    }
}

impl Tree {
    fn new() -> Self {
        Tree { root: None }
    }

    fn get_height(brn: &Option<Box<Data>>) -> i32 {

        match brn {
            None => { 0 }
            Some(n) => { n.height }
        }

    }

    fn get_balance(brn: Option<&Box<Data>>) -> i32 {

        match brn {
            None => { 0 }
            Some(b) => { Self::get_height(&b.right) - Self::get_height(&b.left) }
        }

    }

    fn get_max(val_one: i32, val_two: i32) -> i32 {

        if val_one > val_two {
            return val_one;
        }

        val_two
    }

    fn rotate_right(mut brn: Option<Box<Data>>) -> Option<Box<Data>> {
        let mut new_head: Option<Box<Data>> = if let Some(b) = brn.as_mut() {
            b.left.take()
        } else { None };
        let srt_nodes: Option<Box<Data>> = if let Some(b) = new_head.as_mut() {
            b.right.take()
        } else { None };

        if let Some(b) = brn.as_mut() {
            b.left = srt_nodes;
            b.height = 1 + Self::get_max(Self::get_height(&b.right), Self::get_height(&b.left));
        }

        if let Some(b) = new_head.as_mut() {
            b.right = brn;
            b.height = 1 + Self::get_max(Self::get_height(&b.right), Self::get_height(&b.left));
        }

        new_head
    }

    fn left_rotate(mut brn: Option<Box<Data>>) -> Option<Box<Data>> {
        let mut new_head: Option<Box<Data>> = if let Some(b) = brn.as_mut() {
            b.right.take()
        } else { None };
        let srt_node: Option<Box<Data>> = if let Some(b) = new_head.as_mut() {
            b.left.take()
        } else { None };

        if let Some(b) = brn.as_mut() {
            b.right = srt_node;
            b.height = 1 + Self::get_max(Self::get_height(&b.right), Self::get_height(&b.left));
        }

        if let Some(b) = new_head.as_mut() {
            b.left = brn;
            b.height = 1 + Self::get_max(Self::get_height(&b.right), Self::get_height(&b.left));
        }

        new_head
    }

    fn insert(tree: Option<Box<Data>>, data: Data) -> Option<Box<Data>> {

        match tree {
            None => {
                Some(Box::new(data))
            }
            Some(mut d) => {
                let wh_sid: char;

                if d.freq > data.freq {
                    d.left = Self::insert(d.left, data);
                    wh_sid = 'l';
                } else if d.freq < data.freq {
                    d.right = Self::insert(d.right, data);
                    wh_sid = 'r'
                } else {

                    match d.word.cmp(&data.word) {
                        Ordering::Greater => {
                            d.right = Self::insert(d.right, data);
                            wh_sid = 'r';
                        }
                        Ordering::Less => {
                            d.left = Self::insert(d.left, data);
                            wh_sid = 'l';
                        }
                        Ordering::Equal => {
                            d.left = Self::insert(d.left, data);
                            wh_sid = 'l';
                        }
                    }

                }

                let old_data: Option<&Box<Data>>;

                if wh_sid == 'r' {
                    old_data = d.right.as_ref();
                } else {
                    old_data = d.left.as_ref();
                }

                if let None = old_data {
                    println!("none");
                    return Some(d);
                }

                d.height = 1 + Self::get_max(Self::get_height(&d.right), Self::get_height(&d.left));
                let bal: i32 = Self::get_balance(Some(&d));
                let old_data: &Box<Data> = old_data.unwrap();

                if bal < -1 && old_data.freq < d.freq {
                    return Self::rotate_right(Some(d));
                }
                if bal > 1 && old_data.freq > d.freq {
                    return Self::left_rotate(Some(d));
                }
                if bal < -1 && old_data.freq > d.freq {
                    d.left = Self::left_rotate(d.left.take());
                    return Self::rotate_right(Some(d));
                }
                if bal > 1 && old_data.freq < d.freq {
                    d.right = Self::rotate_right(d.right.take());
                    return Self::left_rotate(Some(d));
                }

                Some(d)
            }

        }

    }

    fn push(&mut self, data: Data) {
        self.root = Self::insert(self.root.take(), data);
    }

    fn show(tree: Option<&Box<Data>>, mut str_data: String) -> String {

        match tree {
            None => { return str_data; }
            Some(b) => {
                str_data = Self::show(b.left.as_ref(), str_data);
                str_data = format!("{} [ Word:{} | Frequency:{} | Height:{} ] -> ", str_data, b.word, b.freq, b.height);
                str_data = Self::show(b.right.as_ref(), str_data);
            }
        }

        str_data
    }

    fn show_tree(&self) -> String {
        Self::show(self.root.as_ref(), String::new())
    }
}

fn main() {
    let mut tree: Tree = Tree::new();
    let mut map: HashMap<String, u32> = HashMap::with_capacity(1000);
    let o_args: Args = env::args();
    let args: Vec<String> = o_args.collect();

    if args.len() > 3 || args.len() == 2 {
        println!("Give {} number of arguments are not allowed either not arguments or 2 arguments", args.len() - 1);
        return;
    }

    let mut file: File = File::open("task.txt").unwrap_or_else(|err| {
        println!("Cannot find file {} ", err);
        process::exit(1);
    });
    let mut content: String = String::new();
    file.read_to_string(&mut content).expect("cannot read data from the file");

    let split_string: SplitWhitespace = content.split_whitespace();

    for s in split_string {
        let mut new_string: String = s.to_string();

        if args.len() > 1 {

            if args[1] == "l" && args[2] == "-y" {
                new_string = new_string.to_lowercase();
            }

        }

        let count = map.entry(new_string).or_insert(0);
        *count += 1;
    }

    for (k, v) in map {
        tree.push(Data::new(k, v));
    }

    let output: String = tree.show_tree();
    println!("The output: {}", output);

    let mut file: File = OpenOptions::new().append(true).create(true).open("output.txt").unwrap_or_else(|err| {
        println!("The problem is: {}", err);
        process::exit(1);
    });
    writeln!(file, "{}", output).unwrap_or_else(|err| {
        println!("Unexpected Error: {}", err);
        process::exit(1);
    });
}
