use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex, MutexGuard};
use std::time::Duration;
use std::thread::{self, JoinHandle};

fn main() {
    let m_res1: Arc<Mutex<i32>> = Arc::new(Mutex::new(0));
    let m_res2: Arc<Mutex<i32>> = Arc::new(Mutex::new(0));

    let (tx, rx): (Sender<_>, Receiver<_>) = mpsc::channel();

    let res1: Arc<Mutex<i32>> = Arc::clone(&m_res1);
    let res2: Arc<Mutex<i32>> = Arc::clone(&m_res2);

    let tx1: Sender<i32> = Sender::clone(&tx);
    let tx2: Sender<i32> = Sender::clone(&tx);

    thread::spawn(move || {
        println!("Trying to access resource one from thread 1");
        let mut val_res1: MutexGuard<'_, i32> = res1.lock().unwrap();
        *val_res1 += 1;
        println!("Accessed resource one: {} from thread 1", *val_res1);
        thread::sleep(Duration::from_secs(1));

        println!("Trying to access resource two from thread 1");
        let mut val_res2: MutexGuard<'_, i32> = res2.lock().unwrap();
        *val_res2 += 1;
        println!("Accessed resource two: {} from thread 1", *val_res2);

        tx1.send(0).unwrap();
    });

    let res1: Arc<Mutex<i32>> = Arc::clone(&m_res1);
    let res2: Arc<Mutex<i32>> = Arc::clone(&m_res2);

    thread::spawn(move || {
        println!("Trying to access resource two from thread 2");
        let mut val_res2: MutexGuard<'_, i32> = res2.lock().unwrap();
        *val_res2 += 1;
        println!("Accessed resource two: {} from thread 2", *val_res2);
        thread::sleep(Duration::from_secs(1));
        
        println!("Trying to access resource one from thread 2");
        let mut val_res1: MutexGuard<'_, i32> = res1.lock().unwrap();
        *val_res1 += 1;
        println!("Accessed resource one: {} from thread 2", *val_res1);

        tx2.send(1).unwrap();
    });

    for i in rx {
        println!("value of rx: {}", i);
    }
}
