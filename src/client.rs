use std::net::TcpStream;
use std::io::{Read, Write};
use std::thread;

pub fn handle_client(stream: TcpStream, target_address: String) {
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
