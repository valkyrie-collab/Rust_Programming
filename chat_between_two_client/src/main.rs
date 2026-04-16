use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;
use std::sync::mpsc;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    println!("Server listening on 127.0.0.1:8080...");

    // Create channels for communication
    let (tx1, rx1) = mpsc::channel();  // Client 1 ↔ Server
    let (tx2, rx2) = mpsc::channel();  // Client 2 ↔ Server

    // Accept Client 1
    let (stream1, addr1) = listener.accept().unwrap();
    println!("Client 1 connected from: {}", addr1);
    thread::spawn(move || {
        handle_client(stream1, tx1, rx2, "Client 1");
    });

    // Accept Client 2
    let (stream2, addr2) = listener.accept().unwrap();
    println!("Client 2 connected from: {}", addr2);
    thread::spawn(move || {
        handle_client(stream2, tx2, rx1, "Client 2");
    });

    println!("Both clients connected! Chat started...");

    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}

fn handle_client(
    mut stream: TcpStream,
    sender: mpsc::Sender<String>,
    receiver: mpsc::Receiver<String>,
    name: &str,
) {
    let mut buffer = [0; 1024];

    loop {
        // READ from client
        match stream.read(&mut buffer) {
            Ok(0) => {
                println!("{} disconnected", name);
                break;
            }
            Ok(n) => {
                let msg = String::from_utf8_lossy(&buffer[0..n]).to_string();
                println!("{} sent: {}", name, msg);
                sender.send(msg).unwrap();
            }
            Err(e) => {
                println!("{} error: {}", name, e);
                break;
            }
        }

        // SEND to client
        if let Ok(msg) = receiver.try_recv() {
            println!("{} sending: {}", name, msg);
            let _ = stream.write_all(msg.as_bytes());
        }
    }
}