use std::net::TcpStream;
use std::sync::mpsc::{ self, Sender, Receiver };
use std::io::{self, BufReader, BufRead, Write, Read, StdinLock, Error };
use std::thread;

fn main() {
    let res_stream: Result<TcpStream, Error> = TcpStream::connect("127.0.0.7:8080");

    match res_stream {
        Ok(mut stream_reader) => {
            let mut stream_writer: TcpStream = stream_reader.try_clone().unwrap();
            let (sx, rx): (Sender<String>, Receiver<String>) = mpsc::channel();

            print!("Enter your username: ");
            io::stdout().flush().unwrap();

            let mut username: String = String::new();
            io::stdin().read_line(&mut username).unwrap();
            username = format!("Username: {}", username.trim());

            stream_writer.write_all(username.as_bytes()).unwrap();

            thread::spawn(move || {
                let mut buffer: [u8; 2048] = [0; 2048];
                // let mut is_disconnect: bool = false;
                // let mut temp_stream_writer: TcpStream = stream_reader.try_clone().unwrap();

                loop {

                    //this stream reader in client reads data from server that other client sends
                    match stream_reader.read(&mut buffer) {
                        Ok(0) => {
                            println!("htere");
                            break;
                        }
                        Ok(n) => {
                            let msg: String = String::from_utf8_lossy(&buffer[0..n]).to_string();
                            // msg = format!("Message from username: {}", msg);
                            sx.send(msg).unwrap();
                        }
                        Err(e) => {
                            println!("Err Occurred: {}", e);
                            break;
                        }
                    }

                    if let Ok(res) = rx.try_recv() {
                        println!("{}", res);
                    }

                }

            });

            let reader: BufReader<StdinLock> = BufReader::new(io::stdin().lock());

            for line in reader.lines() {

                match line {
                    Ok(msg) => {

                        if let Err(e) = stream_writer.write_all(msg.as_bytes()) {
                            println!("Stream Err: {}", e);
                            break;
                        }

                    }
                    Err(e) => {
                        println!("Err: {}", e);
                        break;
                    }
                }

            }
        }
        Err(e) => {
            println!("Cannot connect to port 127.0.0.7:8080: {}", e);
        }
    }

}