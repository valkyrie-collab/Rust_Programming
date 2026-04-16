extern crate multithread_application;

use std::fs::File;
use std::io::{ Read, Write };
use std::net::{ TcpListener, TcpStream };
// use threadpool::ThreadPool;
use multithread_application::ThreadPool;

fn handle_connection(mut stream: TcpStream) {
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
}
fn main() {
    let listener: TcpListener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool: ThreadPool = ThreadPool::new(4);

    for l in listener.incoming() {
        let stream: TcpStream = l.unwrap();
        pool.execute( || { 
            handle_connection(stream);
        });
    }

}
