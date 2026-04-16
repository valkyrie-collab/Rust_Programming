extern crate front_back_list;

use front_back_list::list::{self, List};
// use std::sync::{Arc, Mutex, MutexGuard};
use std::thread::{self, JoinHandle};
use std::time::Duration;

fn back_front_both() {
    let mut list: List<'_> = List::new();
    let str_list: Vec<String> = vec![
        String::from("Rajarshi"), String::from("Shovona")
    ];

    for i in &str_list {
        list.push_back(i);
        thread::sleep(Duration::from_secs(1));
        list.show();
    }

    let mut list: List<'_> = List::new();
    let str_list: Vec<String> = vec![
        String::from("Rajarshi"), String::from("Shovona"), 
        String::from("Arijit"), String::from("Samrat")
    ];

    for i in &str_list {
        list.push_front(i);
        thread::sleep(Duration::from_secs(1));
        list.show();
    }

    println!("FIN\n");
}

fn main() {
    let handle_one:JoinHandle<()> = thread::spawn(|| {
        let mut list: List<'_> = List::new();
        let str_list: Vec<String> = vec![
            String::from("Rajarshi"), String::from("Shovona")
        ];

        for i in &str_list {
            list.push_back(i);
            thread::sleep(Duration::from_secs(1));
            list.show();
        }

        println!("FIN\n")
    });

    let handle_two: JoinHandle<()> = thread::spawn(|| {
        let mut list: List<'_> = List::new();
        let str_list: Vec<String> = vec![
            String::from("Arijit"), String::from("Samrat"), String::from("Ujan")
        ];

        for i in &str_list {
            list.push_front(i);
            thread::sleep(Duration::from_secs(1));
            list.show();
        }

        println!("FIN\n");
    });

    let handle_three: JoinHandle<()> = thread::spawn(back_front_both);

    handle_one.join().unwrap();
    handle_two.join().unwrap();

    println!("handle one and two has been joined successfully");

    handle_three.join().unwrap();
}
