use std::{net::TcpStream, io::Write};

pub struct Client {
    stream: TcpStream
}

impl Client {
    pub fn new() -> Client {
        Client {
            stream: TcpStream::connect("127.0.0.1:7878").unwrap()
        }
    }

    pub fn send(&mut self, msg: &str) {
        self.stream.write(msg.as_bytes()).unwrap();
    }
}