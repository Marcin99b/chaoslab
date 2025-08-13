use core::time;
use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread::{self, JoinHandle};

pub struct Redirection {
    pub listening_port: i32,
    pub target_address: String,
    pub mode: RedirectionMode,
}

#[derive(Clone)]
pub enum RedirectionMode {
    Started,
    Off,
    Slowed(u64),
}

impl Redirection {
    pub fn new(listening_port: i32, target_address: String) -> Redirection {
        Redirection {
            listening_port,
            target_address,
            mode: RedirectionMode::Off,
        }
    }
    pub fn init(&self) -> io::Result<JoinHandle<()>> {
        let listening_port = self.listening_port;
        let target_address = self.target_address.clone();
        let mode = self.mode.clone();
        let t = thread::spawn(move || {
            let address = format!("127.0.0.1:{}", listening_port);
            let listener = TcpListener::bind(address).unwrap();
            for stream in listener.incoming() {
                if let Ok(s) = stream {
                    match mode.clone() {
                        RedirectionMode::Started => {
                            Redirection::handle_client(s, target_address.clone())
                        }
                        RedirectionMode::Slowed(x) => {
                            thread::sleep(time::Duration::from_millis(x));
                            Redirection::handle_client(s, target_address.clone());
                        }
                        RedirectionMode::Off => s.shutdown(std::net::Shutdown::Write).unwrap(),
                    };
                }
            }
        });
        Ok(t)
    }
    pub fn start(&mut self) {
        let _ = std::mem::replace(&mut self.mode, RedirectionMode::Started);
    }

    pub fn stop(&mut self) {
        let _ = std::mem::replace(&mut self.mode, RedirectionMode::Off);
    }

    pub fn slow(&mut self, ms: u64) {
        let _ = std::mem::replace(&mut self.mode, RedirectionMode::Slowed(ms));
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
