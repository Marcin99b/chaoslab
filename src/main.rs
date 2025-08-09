use clap::Parser;
use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    sync::Arc,
    thread,
};

#[derive(Parser)]
struct Cli {
    listen_on: i32,
    target: i32,
}

fn main() -> std::io::Result<()> {
    let params = Cli::parse();
    assert_ne!(
        params.listen_on, params.target,
        "Target port must be different than listening port."
    );

    let address = format!("127.0.0.1:{}", params.listen_on);
    println!("listen_on: {}, target: {}", params.listen_on, params.target);

    let listener = TcpListener::bind(address)?;
    for stream in listener.incoming() {
        handle_client(stream?, params.target);
    }

    Ok(())
}

fn handle_client(stream: TcpStream, target: i32) {
    use std::sync::Mutex;

    let address = format!("127.0.0.1:{}", target);
    let target_stream = Arc::new(Mutex::new(TcpStream::connect(address).unwrap()));
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
