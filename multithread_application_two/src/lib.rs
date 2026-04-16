use std::thread::{self, JoinHandle};
use std::sync::{ Arc, Mutex, MutexGuard };
use std::sync::mpsc::{ self, Sender, Receiver };

trait FnBox {
    fn call_box(self: Box<Self> );
}

impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<Self>) {
        (*self)()
    }
}

type Job = Box<dyn FnBox + Send + 'static>;

enum Message {
    NewJob(Job),
    Terminate
}

struct Worker {
    id: usize,
    worker: Option<JoinHandle<()>>
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Sender<Message>
}

impl Worker {
    fn new(id: usize, rec: Arc<Mutex<Receiver<Message>>>) -> Self {
        let thread: JoinHandle<()> = thread::spawn(move || {

            loop {
                let message: Message = {
                    let receive: MutexGuard<'_, Receiver<Message>> = rec.lock().unwrap();
                    receive.recv().unwrap()
                };

                match message {
                    Message::NewJob(job) => {
                        println!("Doing job of Id: {}", id);
                        job.call_box();
                    }
                    Message::Terminate => {
                        println!("Shutting down the thread: id {}", id);
                        break;
                    }
                }

            }

        });

        Worker { id: id, worker: Some(thread) }
    }
}

impl ThreadPool {
    pub fn new(size: usize) -> Self {
        assert!(size > 0);

        let (sender, receiver): (Sender<Message>, Receiver<Message>) = mpsc::channel();
        let receiver: Arc<Mutex<Receiver<Message>>> = Arc::new(Mutex::new(receiver));
        let mut workers: Vec<Worker> = Vec::with_capacity(size);

        for i in 0..size {
            workers.push(Worker::new(i, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }

    pub fn execute<F: FnOnce() + Send + 'static>(&self, f: F) {
        let job: Box<dyn FnBox + Send + 'static> = Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap()
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Shutting down threads");

        for _ in &mut self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.worker.take() {
                thread.join().unwrap();
            }

        }

    }
}