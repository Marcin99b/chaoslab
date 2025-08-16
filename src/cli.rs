use std::{io::Write, net::TcpStream};

pub fn send(message: String, address: String) {
    println!("sending request {}", message);
    let mut stream = TcpStream::connect(address).unwrap();
    let buf = message.as_bytes();
    stream.write_all(&buf).unwrap();
    stream.shutdown(std::net::Shutdown::Write).unwrap();
}
