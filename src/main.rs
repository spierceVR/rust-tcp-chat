use std::thread;

mod tcp_client;
mod tcp_server;

fn main() {
    let handle = thread::spawn(|| {
        tcp_server::run();
    });
    let mut c1: tcp_client::Client = tcp_client::Client::new();
    c1.send("hello\n");
    c1.send("world\n");
    c1.send("swag\n");
    handle.join().unwrap();
}