use std::{net::TcpStream, io::Write};

pub struct client {
    stream: TcpStream
}

impl client {
    pub fn new() -> client {
        client {
            stream: TcpStream::connect("127.0.0.1:7878").unwrap()
        }
    }

    pub fn send(&mut self, msg: &str) {
        self.stream.write(msg.as_bytes()).unwrap();
    }
}