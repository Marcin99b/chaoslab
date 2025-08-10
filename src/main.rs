use clap::Parser;
use std::{
    collections::HashMap,
    io::{self, Read, Write},
    net::{TcpListener, TcpStream},
    thread::{self, JoinHandle},
};

#[derive(Parser)]
struct Cli {
    listen_on: i32,
    target: i32,
}

fn main() -> io::Result<()> {
    // let params = Cli::parse();
    // assert_ne!(
    //     params.listen_on, params.target,
    //     "Target port must be different than listening port."
    // );

    // println!("listen_on: {}, target: {}", params.listen_on, params.target);
    // let target_address = format!("127.0.0.1:{}", params.target);

    let mut redirections_configs = HashMap::new();
    redirections_configs.insert(1001, "127.0.0.1:7176");
    redirections_configs.insert(1002, "127.0.0.1:5241");

    let mut threads = Vec::new();

    for config in redirections_configs {
        let t = Redirection::new(config.0, config.1.to_string())
            .start()
            .unwrap();
        threads.push(t);
    }

    for t in threads {
        t.join().unwrap();
    }

    Ok(())
}

struct Redirection {
    listening_port: i32,
    target_address: String,
}

impl Redirection {
    fn new(listening_port: i32, target_address: String) -> Redirection {
        Redirection {
            listening_port: listening_port,
            target_address: target_address,
        }
    }

    fn start(&self) -> io::Result<JoinHandle<()>> {
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

fn handle_client(stream: TcpStream, target_address: String) {
    use std::thread;
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
