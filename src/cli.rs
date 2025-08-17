use std::{io::Write, net::TcpStream};

pub fn send(message: String, address: String) {
    println!("sending request {}", message);
    match TcpStream::connect(address) {
        Ok(mut stream) => {
            let buf = message.as_bytes();
            if let Err(e) = stream.write_all(buf) {
                eprintln!("Failed to send request: {}", e);
                return;
            }
            if let Err(e) = stream.shutdown(std::net::Shutdown::Write) {
                eprintln!("Failed to shutdown stream: {}", e);
            }
        }
        Err(e) => {
            eprintln!("Failed to connect: {}", e);
        }
    }
}
