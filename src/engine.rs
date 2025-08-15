use core::str;
use std::{io::Read, net::TcpListener};

use crate::input_param::parse_args_from_string;

pub fn start_engine(address: String) {
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
                }
                Err(_) => break,
            }
        }
    }
}
