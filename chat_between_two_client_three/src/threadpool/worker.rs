use std::thread::{ self, JoinHandle };
use std::sync::{ Arc, Mutex, MutexGuard, mpsc::Receiver };
use crate::threadpool::fnbox::Task;

pub enum Message {
    NewTask(Task),
    Terminate
}

pub struct Worker {
    id: usize,
    worker: Option<JoinHandle<()>>
}

impl Worker {
    pub fn new(worker_id: usize, rec: Arc<Mutex<Receiver<Message>>>) -> Self {
        let personal_worker: JoinHandle<()> = thread::spawn(move || {
            
            loop {
                let msg: Message = {
                    let receiver: MutexGuard<Receiver<Message>> = rec.lock().unwrap();
                    receiver.recv().unwrap()
                };
                
                match msg { 
                    Message::NewTask(task) => {
                        println!("Doing task of id: {}", worker_id);
                        task.call_box();
                    }
                    Message::Terminate => {
                        println!("terminating task with id: {}", worker_id);
                        break;
                    }
                }
                
            }
            
        });
        
        Worker { id: worker_id, worker: Some(personal_worker) }
    }
    
    pub fn get_id(&self) -> usize {
       self.id
    }
    
    pub fn get_worker(&mut self) -> &mut Option<JoinHandle<()>> {
        &mut self.worker
    }
}