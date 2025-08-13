use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread::{self, JoinHandle};

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
                if let Ok(s) = stream {
                    Redirection::handle_client(s, target_address.clone());
                }
            }
        });
        Ok(t)
    }

    fn handle_client(stream: TcpStream, target_address: String) {
        let target_stream = TcpStream::connect(target_address).unwrap();

        let mut client_read = stream.try_clone().unwrap();
        let mut client_write = stream;
        let mut target_read = target_stream.try_clone().unwrap();
        let mut target_write = target_stream;

        // Thread: client -> target
        let t1 = thread::spawn(move || {
            let mut buf = [0u8; 4096];
            loop {
                match client_read.read(&mut buf) {
                    Ok(0) => {
                        let _ = target_write.shutdown(std::net::Shutdown::Write);
                        break;
                    }
                    Ok(n) => {
                        if target_write.write_all(&buf[..n]).is_err() {
                            break;
                        }
                    }
                    Err(_) => break,
                }
            }
        });

        // Thread: target -> client
        let t2 = thread::spawn(move || {
            let mut buf = [0u8; 4096];
            loop {
                match target_read.read(&mut buf) {
                    Ok(0) => {
                        let _ = client_write.shutdown(std::net::Shutdown::Write);
                        break;
                    }
                    Ok(n) => {
                        if client_write.write_all(&buf[..n]).is_err() {
                            break;
                        }
                    }
                    Err(_) => break,
                }
            }
        });

        let _ = t1.join();
        let _ = t2.join();
    }
}
