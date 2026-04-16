mod client_handler;
mod threadpool;

use std::io::{Read, Write};
use std::net::{TcpStream, TcpListener };
use std::sync::{ Arc, Mutex, MutexGuard };
use std::sync::mpsc::{self, Sender, Receiver };
use std::time::Duration;
use crate::threadpool::ThreadPool;
use crate::client_handler::Client;

fn broadcast_message(client: Arc<Mutex<Vec<Client>>>, id: usize, rx: &Receiver<String>) {

    if let Ok(rec) = rx.recv_timeout(Duration::from_millis(100)) {
        let mut clients: MutexGuard<Vec<Client>> = client.lock().unwrap();

        for client in clients.iter_mut() {

            if client.get_id() != id {
                let stream: &mut TcpStream = client.get_stream().unwrap();
                stream.write_all(rec.as_bytes()).unwrap();
                stream.flush().unwrap();
            }

        }

    }

}

pub fn server() {
    //Since server : initialize threadpool with particular number of threads or else infinte number
    //of threads would have been created
    let pool: ThreadPool = ThreadPool::new(4);
    //Making a listener to listen to particular port here: 8080
    let listener: TcpListener = TcpListener::bind("127.0.0.7:8080").unwrap();
    //Made a global like clients what will store all the clients upto 4. upto when server is alive
    let clients: Arc<Mutex<Vec<Client>>> = Arc::new(Mutex::new(Vec::with_capacity(4)));
    //unique server id which will be used as client id also
    let mut server_id: usize = 0;

    println!("Listening to port 8080....");

    for res_stream in listener.incoming() {

        if let Ok(mut stream_reader) = res_stream {
            server_id += 1;
            let (sx, rx): (Sender<String>, Receiver<String>) = mpsc::channel();
            let stream_writer: TcpStream = stream_reader.try_clone().unwrap();
            let clone_clients: Arc<Mutex<Vec<Client>>> = Arc::clone(&clients);
            let mut client: Client = Client::new();
            let mut buffer: [u8; 2048] = [0; 2048];

            pool.execute(move || {
                // println!("server id: {}", server_id);
                let mut username: String = String::from("no_username");

                client.set_id(server_id);
                {
                    let mut current_clients: MutexGuard<Vec<Client>> = clone_clients.lock().unwrap();
                    client.set_stream(stream_writer);
                    current_clients.push(client);
                }

                stream_reader.set_read_timeout(Some(Duration::from_millis(100))).unwrap();

                loop {

                    //this stream reader reads message from clients
                    match stream_reader.read(&mut buffer) {
                        Ok(0) => {
                            // println!("pressed ctrl+d");

                            {
                                let mut clients: MutexGuard<Vec<Client>> = clone_clients.lock().unwrap();
                                let mut k: usize = 0;
                                //k is used as index to remove that particular client from the vector also so
                                //it do no cause errror and memory is little free: main reason it will
                                //not be required later so way keep it

                                for client in clients.iter_mut() {

                                    if client.get_id() == server_id {
                                        // println!("clint = server");

                                        let msg: String = format!("The user with {} has left the grp chat", username);
                                        sx.send(msg).unwrap();

                                        break;
                                    }

                                    k += 1;
                                }

                                clients.remove(k);
                            }

                            broadcast_message(Arc::clone(&clone_clients), server_id, &rx);
                            break;
                        },
                        Ok(n) => {
                            let msg: String = String::from_utf8_lossy(&buffer[0..n]).to_string();
                            sx.send(msg).unwrap();
                        }
                        Err(e) => {
                            println!("Err: {}", e);
                            break;
                        }
                    }

                    if let Ok(mut rec) = rx.recv_timeout(Duration::from_millis(100)) {
                        let mut is_visited: bool = false;
                        let mut clients: MutexGuard<Vec<Client>> = clone_clients.lock().unwrap();

                        if rec.starts_with("Username: ") {
                            is_visited = true;

                            for client in clients.iter_mut() {

                                if client.get_id() == server_id {
                                    // println!("Added username");
                                    let name: &str = &rec[10..];

                                    client.set_username(name.to_string());
                                    username = name.to_string();
                                    rec = format!("User with username: {} has joined the grp", name.to_string());
                                    break;
                                }

                            }

                        }

                        if !is_visited {
                            rec = format!("user {}: {}", username, rec);
                        }

                        for client in clients.iter_mut() {

                            if client.get_id() != server_id {
                                // println!("client id: {} and server id: {}", client.get_id(), server_id);

                                if let None = client.get_username() {
                                    println!("the client with id {} do not have username ", client.get_id());
                                    continue
                                }

                                let new_stream: &mut TcpStream = client.get_stream().unwrap();

                                new_stream.write_all(rec.as_bytes()).unwrap();
                                new_stream.flush().unwrap();
                            }

                        }

                    }

                }

            });
        } else {
            println!("Unexpected error occur");
            break;
        }

    }
}
