use std::net::TcpStream;

pub struct Client {
    id: usize,
    username: Option<String>,
    stream: Option<TcpStream>
}

impl Client {
    pub fn new() -> Self {
        Client { id: 0, username: None, stream: None }
    }
    
    pub fn set_username(&mut self, client_username: String) {
        self.username = Some(client_username);
    }
    
    pub fn set_id(&mut self, client_id: usize) {
        self.id = client_id;
    }
    
    pub fn set_stream(&mut self, stream: TcpStream) {
        self.stream = Some(stream);
    }
    
    pub fn get_id(&self) -> usize {
        self.id
    }
    
    pub fn get_username(&self) -> Option<&String> {
        
        match self.username.as_ref() { 
            None => None,
            Some(s) => Some(s)
        }
        
    }
    
    pub fn get_stream(&mut self) -> Option<&mut TcpStream> {
        
        match self.stream.as_mut() { 
            None => None,
            Some(s) => Some(s)
        }
        
    } 
}