use std::fmt::format;
use std::io::{Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::sync::{mpsc::{self, Sender, Receiver}, Arc, Mutex, MutexGuard};
use std::thread;
use std::time::Duration;

struct Client {
    id: usize,
    username: Option<String>,
    message: Option<String>,
    stream: Option<TcpStream>
}

impl Client {
    fn new() -> Self {
        Client {
            id: 0,
            username: None,
            message: None,
            stream: None
        }
    }

    fn insert_stream(&mut self,st: TcpStream) {
        // self.username = Some(ur);
        self.stream = Some(st);
        // self.message = Some(msg)
    }

    fn insert_username(&mut self, ur: String) {
        self.username = Some(ur);
        // self.stream = Some(st);
        // self.message = Some(msg)
    }

    fn insert_id(&mut self, ur_id: usize) {
        // self.username = Some(ur);
        // self.stream = Some(st);
        self.id = ur_id;
    }
}

fn main() {
    let listener: TcpListener = TcpListener::bind("127.0.0.7:8080").unwrap();
    let streams: Arc<Mutex<Vec<Client>>> = Arc::new(Mutex::new(Vec::new()));
    // let (mut stream, _addr): (TcpStream, SocketAddr) = listener.accept().unwrap();
    let mut buffer: [u8; 200] = [0; 200];
    let mut i: usize = 0;

    for res_stream in listener.incoming() {
        i += 1;

        if let Ok(mut stream) = res_stream {
            let (sx, rx): (Sender<String>, Receiver<String>) = mpsc::channel();
            let streams_clone: Arc<Mutex<Vec<Client>>> = Arc::clone(&streams);
            let mut client: Client = Client::new();
            client.insert_id(i);

            {
                let mut stm: MutexGuard<Vec<Client>> = streams_clone.lock().unwrap();
                client.insert_stream(stream.try_clone().unwrap());
                stm.push(client);
            }

            thread::spawn(move || {

                loop {

                    match stream.read(&mut buffer) {
                        Ok(0) => break,
                        Ok(s) => {
                            println!("doing");
                            let msg: String = String::from_utf8_lossy(&buffer[0..s]).to_string();
                            sx.send(msg).unwrap();
                        }
                        Err(e) => {
                            println!("{}", e);
                            break;
                        }
                    }

                    if let Ok(mut rec) = rx.try_recv() {
                        let mut stm: MutexGuard<Vec<Client>> = streams_clone.lock().unwrap();

                        for wt_stm in stm.iter_mut() {

                            if rec.starts_with("Username: ") {
                                println!("Got username and id {}", i);
                                wt_stm.insert_username(rec[9..].to_string());
                                rec = format!("User joind with username: {}", wt_stm.username.as_ref().unwrap());
                            }

                            if wt_stm.id != i {
                                println!("not match id {} and usrid: {}", i, wt_stm.id);
                                wt_stm.stream.as_mut().unwrap().write_all(rec.as_bytes()).unwrap();
                            }

                        }

                    }

                }

            });

        }

    }

}