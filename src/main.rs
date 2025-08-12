use std::{
    env::{self, Args},
    io::{self, Read, Write},
    net::{TcpListener, TcpStream},
    thread::{self, JoinHandle},
};

#[derive(Debug)]
struct InputParam {
    name: String,
    expose: i32,
    target: String,
}

fn main() -> io::Result<()> {
    //example
    // my_app_https=1001:127.0.0.1:7176 my_app_http=1002:127.0.0.1:5241
    // app_name=expose_port:target_address
    let params: Vec<InputParam> = parse_params(env::args());

    println!("{:?}", params);

    let mut threads = Vec::new();
    for param in params {
        let t = Redirection::new(param.expose, param.target.clone())
            .start()
            .unwrap();
        threads.push(t);
    }

    for t in threads {
        t.join().unwrap();
    }

    Ok(())
}

fn parse_params(args: Args) -> Vec<InputParam> {
    args.filter_map(|arg| {
        let eq = arg.find('=')?;
        let name = &arg[..eq];
        let mapping = &arg[eq + 1..];
        let colon = mapping.find(':')?;
        let expose = &mapping[..colon];
        let target = &mapping[colon + 1..];
        let expose_port = expose.parse().ok()?;
        Some(InputParam {
            name: name.to_string(),
            expose: expose_port,
            target: target.to_string(),
        })
    })
    .collect()
}

struct Redirection {
    listening_port: i32,
    target_address: String,
}

impl Redirection {
    fn new(listening_port: i32, target_address: String) -> Redirection {
        Redirection {
            listening_port: listening_port,
            target_address: target_address,
        }
    }

    fn start(&self) -> io::Result<JoinHandle<()>> {
        let listening_port = self.listening_port;
        let target_address = self.target_address.clone();

        let t = thread::spawn(move || {
            let address = format!("127.0.0.1:{}", listening_port);
            let listener = TcpListener::bind(address).unwrap();
            for stream in listener.incoming() {
                handle_client(stream.unwrap(), target_address.clone());
            }
        });
        Ok(t)
    }
}

fn handle_client(stream: TcpStream, target_address: String) {
    use std::thread;
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
