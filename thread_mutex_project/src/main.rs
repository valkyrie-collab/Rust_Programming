// use std::rc::Rc;
use std::sync::{Mutex, MutexGuard, Arc};
use std::thread::{self, JoinHandle};

fn main() {
    let counter: Arc<Mutex<i32>> = Arc::new(Mutex::new(0));
    let mut handles: Vec<JoinHandle<()>> = vec![];

    for _ in 0..10 {
        let counter: Arc<Mutex<i32>> = Arc::clone(&counter);
        let handle: JoinHandle<()> = thread::spawn(move || {
            let mut num: MutexGuard<'_, i32> = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    // let handle: JoinHandle<()> = thread::spawn(move || {
    //     let mut num: MutexGuard<'_, i32> = counter.lock().unwrap();
    //     *num += 1;
    // });
    // handles.push(handle);

    // let handle2: JoinHandle<()> = thread::spawn(move || {
    //     let mut num2 = counter.lock().unwrap();
    //     *num2 += 1;
    // });

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap())
}
