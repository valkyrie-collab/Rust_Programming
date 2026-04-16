use crate::threadpool::worker::{ Message, Worker };
use std::sync::{ Arc, Mutex, mpsc::{ Sender, Receiver, self } };

mod worker;
mod fnbox;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Sender<Message>
}

impl ThreadPool {
    pub fn new(pool_size: usize) -> Self {
        assert!(pool_size > 0);

        let (sx, rx): (Sender<Message>, Receiver<Message>) = mpsc::channel();
        let receiver: Arc<Mutex<Receiver<Message>>> = Arc::new(Mutex::new(rx));
        let mut personal_workers: Vec<Worker> = Vec::with_capacity(4);

        for i in 0..pool_size {
            personal_workers.push(Worker::new(i, Arc::clone(&receiver)));
        }

        ThreadPool { workers: personal_workers, sender: sx }
    }

    pub fn execute<F: FnOnce() + Send + 'static>(&self , function: F) {
        self.sender.send(Message::NewTask(Box::new(function))).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Shutting down threads....");

        for _ in &mut self.workers {
           self.sender.send(Message::Terminate).unwrap();
        }

        for i in &mut self.workers {
            println!("Terminating thread with id: {}", i.get_id());

            if let Some(w) = i.get_worker().take() {
                w.join().unwrap();
            }

        }

    }
}