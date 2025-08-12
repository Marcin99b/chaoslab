mod client;
mod input_param;
mod redirection;

use input_param::parse_args;
use redirection::Redirection;
use std::io;

fn main() -> io::Result<()> {
    //example
    // my_app_https=1001:127.0.0.1:7176 my_app_http=1002:127.0.0.1:5241
    // app_name=expose_port:target_address
    let params = parse_args();

    println!("{:?}", params);

    let mut threads = Vec::new();
    for param in params {
        println!(
            "Start redirection {} | 127.0.0.1:{} -> {}",
            param.name, param.expose, param.target
        );
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
