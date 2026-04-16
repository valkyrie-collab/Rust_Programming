use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::fs::File;

fn handle_connect(mut stream: TcpStream) {
    let mut buffer: [u8; 512] = [0; 512];
    let get: &[u8; 16] = b"GET / HTTP/1.1\r\n";
    let mut file: File;

    stream.read(&mut buffer).unwrap();

    if buffer.starts_with(get) {
        file = File::open("hello.html").unwrap();
    } else {
        file = File::open("404.html").unwrap();
    }

    let mut content: String = String::new();
    
    file.read_to_string(&mut content).unwrap();

    let response: String = format!("HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}", content.len(), content);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();

    // println!("{:?}", buffer);
}

fn main() {
    let listener: TcpListener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        println!("connected to 127.0.0.1:7878");

        let stream: TcpStream = stream.unwrap();
        handle_connect(stream);
    }

}
