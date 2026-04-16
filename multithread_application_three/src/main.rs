extern crate multithread_application_three;

use multithread_application_three::ThreadPool;
use std::thread;
use std::time::Duration;

fn main() {
    let pool: ThreadPool = ThreadPool::new(4);

    pool.execute(|| {
        thread::sleep(Duration::from_secs(1));
        println!("1Hello, world!");
    });
    pool.execute(|| {
        thread::sleep(Duration::from_secs(1));
        println!("2Hello, world!");
    });
    pool.execute(|| {
        thread::sleep(Duration::from_secs(1));
        println!("3Hello, world!");
    });
    pool.execute(|| {
        thread::sleep(Duration::from_secs(1));
        println!("4Hello, world!");
    });
    pool.execute(|| {
        thread::sleep(Duration::from_secs(1));
        println!("5Hello, world!");
    });
    pool.execute(|| {
        thread::sleep(Duration::from_secs(1));
        println!("6Hello, world!");
    });
    pool.execute(|| {
        thread::sleep(Duration::from_secs(1));
        println!("7Hello, world!");
    });
    pool.execute(|| {
        thread::sleep(Duration::from_secs(1));
        println!("8Hello, world!");
    });
    pool.execute(|| {
        thread::sleep(Duration::from_secs(1));
        println!("9Hello, world!");
    });
}
