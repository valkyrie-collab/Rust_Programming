extern crate multithread_application_two;

use multithread_application_two::ThreadPool;
use std::{thread, time::Duration};

fn main() {
    let pool: ThreadPool = ThreadPool::new(5);

    pool.execute(|| {
        thread::sleep(Duration::from_secs(1));
        println!("job0");
    });
    pool.execute(|| {
        thread::sleep(Duration::from_secs(1));
        println!("job1");
    });
    pool.execute(|| {
        thread::sleep(Duration::from_secs(1));
        println!("job2");
    });
    pool.execute(|| {
        thread::sleep(Duration::from_secs(1));
        println!("job3");
    });
    pool.execute(|| {
        thread::sleep(Duration::from_secs(1));
        println!("job4");
    });
    pool.execute(|| {
        thread::sleep(Duration::from_secs(1));
        println!("job5");
    });
    pool.execute(|| {
        thread::sleep(Duration::from_secs(1));
        println!("job6");
    });
}
