use std::thread::{self, JoinHandle};
use std::time::Duration;
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Mutex, MutexGuard};

fn main() {
    // let handle: JoinHandle<()> = thread::spawn(|| {

    //     for i in 1..10 {
    //         println!("Hi the number {} form the spawn thread!", i);
    //         thread::sleep(Duration::from_millis(200));
    //     }

    // });

    // handle.join().unwrap();

    // for i in 1..5 {
    //     println!("Hi the number {} from main thread!", i);
    //     thread::sleep(Duration::from_millis(200));
    // }

    // handle.join().unwrap();
    
    // let v: Vec<i32> = vec![1, 2, 3];

    // let handle: JoinHandle<()> = thread::spawn(move || {
    //     println!("Heres a vector {:?}", v);
    // });


    // handle.join().unwrap();

    // let (tx, rx): (Sender<_>, Receiver<_>) = mpsc::channel();
    // let tx1: Sender<_> = mpsc::Sender::clone(&tx);

    // thread::spawn(move || {
    //     let val: String = String::from("hi");
    //     tx.send(val).unwrap();
    //     // println!("{}", val);
    // });

    // thread::spawn(move || {
    //     let vals: Vec<String> = vec![
    //         String::from("hi"),
    //         String::from("from"),
    //         String::from("the"),
    //         String::from("thread")
    //     ];

    //     for val in vals {
    //         tx1.send(val).unwrap();
    //         thread::sleep(Duration::from_secs(1));
    //     }
    // });
    
    // thread::spawn(move || {
    //     let vals: Vec<String> = vec![
    //         String::from("more"),
    //         String::from("message"),
    //         String::from("for"),
    //         String::from("you")
    //     ];

    //     for val in vals {
    //         tx.send(val).unwrap();
    //         thread::sleep(Duration::from_secs(1));
    //     }
    // });

    // let received: String = rx.recv().unwrap();
    // println!("Got: {}", received);

    // for received in rx {
    //     println!("Got: {}", received);
    // }

    let m: Mutex<i32> = Mutex::new(5);

    {
        let mut num: MutexGuard<'_, i32> = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m);
}

