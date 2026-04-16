# First we need Listener
- To access listener we do it like thi 
```rust
    let listener: TcpListener = TcpListener::bind("127.0.0.7:8080").unwrap();
    for stream in listener.incomming() {} -> this returns Result<Stream>
```

# Q.what is channel and how it is used 

# channel mainly has two parameter SENDER and RECEIVER from my exp upto 29May 2026 I can say
- `Sender<Generic Value>` It mainly use to send data from a thread to another where it has its receiver
- `Receiver<Generic Value>` Gets what is sent from sender of that thread and many represent as rx
## its written as let (sx, rx): (Sender<String>, Receiver<String>) = mpsc::channel(); where mpsc is a part of std::sync::mpsc

# When ever a new thread of Stream TcpStream is created and linked to the server it can only send data from itself to server only
- `stream.read(buffer)` generally use to read data from stream in u8 format and return Result, so the buffer is also u8
- `stream.write_all(<it must be bytes>).unwrap` generally use to write data into the stream so that `stream.read` can read data and it returns void Result

## That means if we write something is stream and access in listener.incomming() that stream will be particular to that client and modification done in that stream will be reflected to that client only to actually send it to the other client we must store the streams of each client and with each reponse write into that stream of the client which is stored in the array in this way the streams of clients that are stored will be written
