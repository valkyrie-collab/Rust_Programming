use std::io::{BufRead, BufReader, Read, StdinLock, Write};
use std::net::TcpStream;
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;
use std::time::Duration;

// struct Client{
//     username: String,
//     message: Option<String>,
//     stream: Option<TcpStream>
// }
// 
// impl Client {
//     fn new(client_username: String) -> Self {
//         Client { username: client_username, message: None, stream: None }
//     }
//     
//     fn insert_msg(&mut self, client_message: String) {
//         self.message = Some(client_message);
//     }
// }

fn main() {
    let mut stream: TcpStream = TcpStream::connect("127.0.0.7:8080").unwrap();
    
    println!("Enter your name");
    let mut client_username: String = String::new();
    std::io::stdin().read_line(&mut client_username).unwrap();
    // let mut client: Client = Client::new(client_username);
    let client_username: String = format!("Username: {}", client_username);
    
    let (sx, rx): (Sender<String>, Receiver<String>) = mpsc::channel();
    let mut buffer: [u8; 200] = [0; 200];
    let mut stream_writer: TcpStream = stream.try_clone().unwrap();
    
    stream_writer.write_all(client_username.as_bytes()).unwrap();

    thread::spawn(move || {

        loop {
            match stream.read(&mut buffer) {
                Ok(0) => break,
                Ok(n) => {
                    let msg: String = String::from_utf8_lossy(&buffer[0..n]).to_string();
                    println!("message from server: {}", msg);
                    let msg: String = format!("the is from client {}", msg);
                    sx.send(msg).unwrap();
                }
                Err(e) => {
                    println!("Err {}", e);
                    break;
                }
            }

            if let Ok(r) = rx.try_recv() {
                println!("{}", r);
            }
        }

    });

    let reader: BufReader<StdinLock> = BufReader::new(std::io::stdin().lock());

    for line in reader.lines() {
        
        match line {
            Ok(l) => {
                if let Err(e) = stream_writer.write_all(l.as_bytes()) {
                    println!("Err: {}", e);
                    break;
                }
            }
            Err(e) => {
                println!("Err: in line {}", e);
                break;
            }
        }

    }
}