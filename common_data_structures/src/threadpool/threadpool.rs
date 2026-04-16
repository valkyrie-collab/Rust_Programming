use crate::threadpool::worker::worker::{Message, Worker};
use crate::threadpool::worker::fnbox::FnBox;
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex};

pub struct ThreadPool {
    pub workers: Vec<Worker>,
    pub sender: Sender<Message>
}

impl ThreadPool {
    pub fn new(pool_size: usize) -> Self {
        let (sx, rx): (Sender<Message>, Receiver<Message>) = mpsc::channel();
        let rec: Arc<Mutex<Receiver<Message>>> = Arc::new(Mutex::new(rx));
        let mut personal_workers: Vec<Worker> = Vec::with_capacity(pool_size);

        for i in 0..pool_size {
            println!("Setting worker with id: {}", i);
            personal_workers.push(Worker::new(i, Arc::clone(&rec)));
        }

        ThreadPool {workers: personal_workers, sender: sx}
    }

    pub fn execute<F: FnBox + Send + 'static>(&self, fnc: F) {
        self.sender.send(Message::NewTask(Box::new(fnc))).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Shutting down ThreadPool");

        for w in self.workers.iter() {
            println!("Sending Termination signal for worker with id: {}", w.value);

            self.sender.send(Message::Terminate).unwrap();
        }

        for worker in self.workers.iter_mut() {
            println!("Shutting down worker with id: {}", worker.value);

            if let Some(w) = worker.worker.take() {
                w.join().unwrap();
            }

        }

    }
}