use clap::Parser;
use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    sync::Arc,
    thread,
};

#[derive(Parser)]
struct Cli {
    port: i32,
}

fn main() -> std::io::Result<()> {
    let port = Cli::try_parse().map_or(1234, |x| x.port);
    println!("port: {}", port);
    let address = format!("127.0.0.1:{}", port);
    println!("address: {}", address);
    let listener = TcpListener::bind(address)?;
    for stream in listener.incoming() {
        handle_client(stream?);
    }

    Ok(())
}

fn handle_client(stream: TcpStream) {
    use std::sync::Mutex;

    let target_stream = Arc::new(Mutex::new(TcpStream::connect("127.0.0.1:7176").unwrap()));
    let client_stream = Arc::new(Mutex::new(stream));

    // Proxy client -> target
    let t1 = {
        let client = Arc::clone(&client_stream);
        let target = Arc::clone(&target_stream);
        thread::spawn(move || {
            let mut buf = [0u8; 4096];
            loop {
                let mut client = client.lock().unwrap();
                match client.read(&mut buf) {
                    Ok(0) => {
                        let target = target.lock().unwrap();
                        let _ = target.shutdown(std::net::Shutdown::Write);
                        break;
                    }
                    Ok(n) => {
                        let mut target = target.lock().unwrap();
                        if target.write_all(&buf[..n]).is_err() {
                            break;
                        }
                    }
                    Err(_) => break,
                }
            }
        })
    };

    let t2 = {
        let client = Arc::clone(&client_stream);
        let target = Arc::clone(&target_stream);
        thread::spawn(move || {
            let mut buf = [0u8; 4096];
            loop {
                let mut target = target.lock().unwrap();
                match target.read(&mut buf) {
                    Ok(0) => {
                        let client = client.lock().unwrap();
                        let _ = client.shutdown(std::net::Shutdown::Write);
                        break;
                    }
                    Ok(n) => {
                        let mut client = client.lock().unwrap();
                        if client.write_all(&buf[..n]).is_err() {
                            break;
                        }
                    }
                    Err(_) => break,
                }
            }
        })
    };

    let _ = t1.join();
    let _ = t2.join();
}
