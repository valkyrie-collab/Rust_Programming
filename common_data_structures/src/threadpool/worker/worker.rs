use std::thread::{self, JoinHandle};
use std::sync::mpsc::Receiver;
use std::sync::{Arc, Mutex, MutexGuard};
use crate::threadpool::worker::fnbox::Task;

pub enum Message {
    NewTask(Task),
    Terminate
}

pub struct Worker {
    pub value: usize,
    pub worker: Option<JoinHandle<()>>
}

impl Worker {
    pub fn new(val: usize, rec: Arc<Mutex<Receiver<Message>>>) -> Self {
        let personal_worker: JoinHandle<()> = thread::spawn(move || {

            loop {
                let message: Message = {
                    let temp: MutexGuard<Receiver<Message>> = rec.lock().unwrap();
                    temp.recv().unwrap()
                };
 
                match message {
                    Message::NewTask(task) => {
                        task.call_box();
                    }
                    Message::Terminate => {
                        println!("Terminating task with id: {}", val);
                        break;
                    }
                }

            }

        });

        Worker {value: val, worker: Some(personal_worker)}
    }
}