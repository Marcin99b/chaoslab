use core::str;
use std::{io::Read, net::TcpListener, thread::JoinHandle};

use crate::{input_param::parse_args_from_string, redirection::Redirection};

#[derive(Debug)]
pub struct Engine {
    threads: Vec<JoinHandle<()>>,
    redirections: Vec<Redirection>,
}

impl Engine {
    pub fn new() -> Engine {
        Engine {
            threads: Vec::new(),
            redirections: Vec::new(),
        }
    }

    pub fn start(&mut self, address: String) {
        let listener = TcpListener::bind(address).unwrap();
        for stream in listener.incoming() {
            let mut buf = [0u8; 4096];
            let mut client_read = stream.unwrap().try_clone().unwrap();
            loop {
                match client_read.read(&mut buf) {
                    Ok(0) => {
                        let _ = client_read.shutdown(std::net::Shutdown::Write);
                    }
                    Ok(n) => {
                        let request = str::from_utf8(&buf[..n]).unwrap();
                        println!("{}", request);
                        let params = parse_args_from_string(request.to_string());

                        for param in params {
                            println!(
                                "Start redirection {} | 127.0.0.1:{} -> {}",
                                param.name, param.expose, param.target
                            );
                            let r = Redirection::new(param.expose, param.target.clone());
                            let t = r.init().unwrap();
                            self.threads.push(t);

                            r.start();
                            r.slow(200);
                            self.redirections.push(r);
                        }
                    }
                    Err(_) => break,
                }
            }
        }
    }
}
