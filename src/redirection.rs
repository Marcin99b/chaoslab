use core::time;
use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};

#[derive(Debug, Clone)]
pub struct Redirection {
    pub name: String,
    pub listening_port: i32,
    pub target_address: String,
    pub mode: Arc<Mutex<RedirectionMode>>,
}

#[derive(Debug, Clone)]
pub enum RedirectionMode {
    Started,
    Off,
    Slowed(u64),
}

impl Redirection {
    pub fn new(name: String, listening_port: i32, target_address: String) -> Redirection {
        Redirection {
            name,
            listening_port,
            target_address,
            mode: Arc::new(Mutex::new(RedirectionMode::Off)),
        }
    }
    pub fn init(&self) -> io::Result<JoinHandle<()>> {
        let listening_port = self.listening_port;
        let target_address = self.target_address.clone();
        let mode = Arc::clone(&self.mode);
        let t = thread::spawn(move || {
            let address = format!("127.0.0.1:{}", listening_port);
            let listener = TcpListener::bind(address).unwrap();
            for stream in listener.incoming() {
                if let Ok(s) = stream {
                    let res = match mode.lock().unwrap().clone() {
                        RedirectionMode::Started => {
                            Redirection::handle_client(s, target_address.clone())
                        }
                        RedirectionMode::Slowed(x) => {
                            thread::sleep(time::Duration::from_millis(x));
                            Redirection::handle_client(s, target_address.clone())
                        }
                        RedirectionMode::Off => {
                            let _ = s.shutdown(std::net::Shutdown::Write);
                            Ok(())
                        }
                    };
                    if let Err(e) = res {
                        eprintln!("Error handling client: {}", e);
                    }
                }
            }
        });
        Ok(t)
    }

    pub fn start(&self) {
        let mut mode = self.mode.lock().unwrap();
        *mode = RedirectionMode::Started;
    }

    pub fn stop(&self) {
        let mut mode = self.mode.lock().unwrap();
        *mode = RedirectionMode::Off;
    }

    pub fn slow(&self, ms: u64) {
        let mut mode = self.mode.lock().unwrap();
        *mode = RedirectionMode::Slowed(ms);
    }

    fn handle_client(stream: TcpStream, target_address: String) -> io::Result<()> {
        let target_stream = TcpStream::connect(target_address)?;

        let mut client_read = stream.try_clone()?;
        let mut client_write = stream;
        let mut target_read = target_stream.try_clone()?;
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
        Ok(())
    }
}
