mod input_param;
mod redirection;

use input_param::parse_args;
use redirection::Redirection;
use std::io;

fn main() -> io::Result<()> {
    let params = parse_args();

    println!("{:?}", params);

    let mut threads = Vec::new();
    let mut redirections = Vec::new();
    for param in params {
        println!(
            "Start redirection {} | 127.0.0.1:{} -> {}",
            param.name, param.expose, param.target
        );
        let r = Redirection::new(param.expose, param.target.clone());
        let t = r.start()?;
        threads.push(t);
        redirections.push(r);
    }

    for t in threads {
        t.join().unwrap();
    }

    Ok(())
}
