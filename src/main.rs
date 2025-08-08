use clap::Parser;
use std::{
    io::{BufRead, BufReader, Read, Write},
    net::{TcpListener, TcpStream},
    str,
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

fn handle_client(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    for item in http_request {
        println!("{}", item);
    }

    let response = "HTTP/1.1 200 OK\r\n\r\n";

    stream.write_all(response.as_bytes()).unwrap();
}
