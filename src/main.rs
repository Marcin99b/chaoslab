mod engine;
mod input_param;
mod redirection;

use redirection::Redirection;
use std::io;

use crate::{engine::Engine, input_param::parse_args_from_console};

fn main() -> io::Result<()> {
    let params = parse_args_from_console();

    println!("{:?}", params);

    Engine::new().start("127.0.0.1:9900".to_string());

    let mut threads = Vec::new();
    let mut redirections = Vec::new();
    for param in params {
        println!(
            "Start redirection {} | 127.0.0.1:{} -> {}",
            param.name, param.expose, param.target
        );
        let r = Redirection::new(param.expose, param.target.clone());
        let t = r.init()?;
        threads.push(t);

        r.start();
        r.slow(200);
        redirections.push(r);
    }

    for t in threads {
        t.join().unwrap();
    }

    Ok(())
}
