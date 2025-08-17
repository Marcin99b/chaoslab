pub struct CommandIter {
    listener: TcpListener,
}

impl Iterator for CommandIter {
    type Item = ParsedCommand;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.listener.incoming().next() {
                Some(Ok(mut stream)) => {
                    let mut buf = [0u8; 4096];
                    match stream.read(&mut buf) {
                        Ok(0) => continue,
                        Ok(n) => match str::from_utf8(&buf[..n]) {
                            Ok(request) => match ParsedCommand::from_str(request) {
                                Ok(cmd) => return Some(cmd),
                                Err(_) => continue,
                            },
                            Err(_) => continue,
                        },
                        Err(_) => continue,
                    }
                }
                Some(Err(_)) => continue,
                None => return None,
            }
        }
    }
}
use std::{io::Read, net::TcpListener, str};

use crate::input_param::ParsedCommand;

pub struct CommandReceiver {
    address: String,
}

impl CommandReceiver {
    pub fn new(address: String) -> Self {
        Self { address }
    }

    pub fn listen(&self) -> CommandIter {
        let listener = TcpListener::bind(&self.address).expect("Failed to bind TCP listener");
        CommandIter { listener }
    }
}
