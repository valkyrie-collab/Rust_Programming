use std::fs::File;
use std::io::{prelude::*, Read};
use std::net::{TcpListener, TcpStream};

fn handle_connection(mut stream: TcpStream) {
    let mut buffer: [u8; 512] = [0; 512];
    
    // Read the request
    stream.read(&mut buffer).unwrap();
    
    // ✅ FIXED: Use uppercase "GET"
    let get_request = b"GET / HTTP/1.1\r\n";
    
    if buffer.starts_with(get_request) {
        // ✅ FIXED: Read and send the file content
        let mut file: File = File::open("hello.html").unwrap();
        let mut contents: String = String::new();
        file.read_to_string(&mut contents).unwrap();
        
        // ✅ FIXED: Include Content-Length header
        let response: String = format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
            contents.len(),
            contents
        );
        
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
        
        println!("✅ Sent hello.html");
    } else {
        // 404 handler
        let mut file: File = File::open("404.html").unwrap();
        let mut contents: String = String::new();
        file.read_to_string(&mut contents).unwrap();
        
        // ✅ FIXED: Include Content-Length header
        let response: String = format!(
            "HTTP/1.1 404 NOT FOUND\r\nContent-Length: {}\r\n\r\n{}",
            contents.len(),
            contents
        );
        
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
        
        println!("✅ Sent 404.html");
    }
}

fn main() {
    let listener: TcpListener = TcpListener::bind("127.0.0.1:7878").unwrap();
    println!("Server running on http://127.0.0.1:7878");
    
    for stream in listener.incoming() {
        let stream: TcpStream = stream.unwrap();
        println!("Connection established!");
        handle_connection(stream);
    }
}
