use std::thread::{ self, JoinHandle };
use std::sync::{ Arc, Mutex, MutexGuard };
use std::sync::mpsc::{ self, Sender, Receiver };

trait FnBox {
    fn call_box(self: Box<Self>);
}

//FnBox is a wrapper of FnOnce which dynamically making rust understand that
//F carries value of the given closer or function and as a result its size become known
//to us and rust lets us call the function

impl <F: FnOnce()> FnBox for F {
    fn call_box(self: Box<Self>) {
        (*self)();
    }
}

type Job = Box<dyn FnBox + Send + 'static>;

enum Message {
    Task(Job),
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
                    let rec: MutexGuard<'_, Receiver<Message>> = rec.lock().unwrap();
                    rec.recv().unwrap()
                };

                match message {
                    Message::Task(t) => {
                        println!("Executing Job {}", id);
                        t.call_box();
                    }
                    Message::Terminate => {
                        println!("Terminating Job {}", id);
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

        let (tx, rx): (Sender<Message>, Receiver<Message>) = mpsc::channel();
        let rec: Arc<Mutex<Receiver<Message>>> = Arc::new(Mutex::new(rx));
        let mut wrs: Vec<Worker> = Vec::with_capacity(size);

        for id in 0..size {
            wrs.push(Worker::new(id, Arc::clone(&rec)));
        }

        ThreadPool { workers: wrs, sender: tx}
    }

    pub fn execute<F: FnOnce() + Send + 'static>(&self, f: F) {
        let job: Box<F> = Box::new(f);
        self.sender.send(Message::Task(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Shutting down the threads");

        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        for w in &mut self.workers {
            println!("Shutting down worker {}", w.id);

            if let Some(t) = w.worker.take() {
                t.join().unwrap();
            }

        }

    }
}