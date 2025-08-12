use std::io;
use std::net::TcpListener;
use std::thread::{self, JoinHandle};
use crate::client::handle_client;

pub struct Redirection {
    pub listening_port: i32,
    pub target_address: String,
}

impl Redirection {
    pub fn new(listening_port: i32, target_address: String) -> Redirection {
        Redirection {
            listening_port,
            target_address,
        }
    }

    pub fn start(&self) -> io::Result<JoinHandle<()>> {
        let listening_port = self.listening_port;
        let target_address = self.target_address.clone();
        let t = thread::spawn(move || {
            let address = format!("127.0.0.1:{}", listening_port);
            let listener = TcpListener::bind(address).unwrap();
            for stream in listener.incoming() {
                handle_client(stream.unwrap(), target_address.clone());
            }
        });
        Ok(t)
    }
}
