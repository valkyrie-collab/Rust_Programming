use std::net::TcpStream;
use std::io::{Read, Write, BufRead, BufReader};
use std::thread;

fn main() {
    // Connect to server
    match TcpStream::connect("127.0.0.1:8080") {
        Ok(mut stream) => {
            println!("✓ Connected to server!");

            let mut stream_read = stream.try_clone().unwrap();

            // Thread for reading from server
            thread::spawn(move || {
                let mut buffer = [0; 1024];
                loop {
                    println!("1.coming here");
                    match stream_read.read(&mut buffer) {
                        Ok(0) => {
                            println!("Server disconnected");
                            break;
                        }
                        Ok(n) => {
                            let msg = String::from_utf8_lossy(&buffer[0..n]);
                            println!("Server: {}", msg);
                        }
                        Err(e) => {
                            println!("Error reading: {}", e);
                            break;
                        }
                    }
                }
            });

            // Main thread for writing to server
            println!("2.coming here");
            let stdin = std::io::stdin();
            let reader = BufReader::new(stdin.lock());
            println!("reader: {:?}", reader);

            for line in reader.lines() {
                match line {
                    Ok(msg) => {
                        if let Err(e) = stream.write_all(msg.as_bytes()) {
                            println!("Error sending: {}", e);
                            break;
                        }
                    }
                    Err(e) => {
                        println!("Error reading input: {}", e);
                        break;
                    }
                }
            }
        }
        Err(e) => {
            println!("✗ Failed to connect: {}", e);
        }
    }
}