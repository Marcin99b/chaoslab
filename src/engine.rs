use core::str;
use std::{
    io::Read,
    net::TcpListener,
    sync::{Arc, Mutex},
    thread::{self, JoinHandle},
};

use crate::{input_param::parse_args_from_string, redirection::Redirection};

#[derive(Debug)]
pub struct Engine {
    threads: Arc<Mutex<Vec<JoinHandle<()>>>>,
    redirections: Arc<Mutex<Vec<Redirection>>>,
}

impl Engine {
    pub fn new() -> Engine {
        Engine {
            threads: Arc::new(Mutex::new(Vec::new())),
            redirections: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn start(&self, address: String) -> JoinHandle<()> {
        let threads = Arc::clone(&self.threads);
        let redirections = Arc::clone(&self.redirections);
        thread::spawn(move || {
            println!("starting engine {}", address);
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
                            println!("Incoming {}", request);
                            if let Ok(command) = parse_args_from_string(request.to_string()) {
                                match command.name.as_str() {
                                    "start" => {
                                        let mut args = command.args.iter();
                                        let r = Redirection::new(
                                            args.next().unwrap().to_string(),
                                            args.next().unwrap().parse().unwrap(),
                                            args.next().unwrap().to_string(),
                                        );
                                        let t = r.init().unwrap();
                                        r.start();
                                        redirections.lock().unwrap().push(r);
                                        threads.lock().unwrap().push(t);
                                    }
                                    "stop" => {
                                        let args = &mut command.args.iter();
                                        let name = args.next().unwrap().to_string();
                                        let redirs = redirections.lock().unwrap();
                                        let r = redirs.iter().find(|x| x.name == name).unwrap();
                                        r.stop();
                                    }
                                    _ => {
                                        println!("Unknown command: {}", command.name);
                                    }
                                };
                            }
                        }
                        Err(_) => break,
                    }
                }
            }
        })
    }
}
