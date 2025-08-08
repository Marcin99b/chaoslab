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
    let http_request = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect::<Vec<_>>()
        .join("\r\n");
    let http_request = format!("{}\r\n\r\n", http_request);

    println!("{}", http_request);

    let mut send_stream = TcpStream::connect("127.0.0.1:7176").unwrap();
    send_stream.write_all(http_request.as_bytes()).unwrap();

    let mut response = String::new();
    let mut buf_reader_send = BufReader::new(&send_stream);
    buf_reader_send.read_to_string(&mut response).unwrap();

    println!("{}", response);

    stream.write_all(response.as_bytes()).unwrap();
}
