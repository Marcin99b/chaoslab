mod cli;
mod command_handler;
mod command_reveiver;
mod engine;
mod input_param;
mod redirection;

use std::io;

use crate::engine::Engine;

fn main() -> io::Result<()> {
    let address = "127.0.0.1:9900".to_string();

    let result = match std::env::args().nth(1) {
        Some(x) => match x.as_str() {
            "engine" => Some(Engine::new().start(address)),
            _ => {
                if x.len() > 3 {
                    cli::send(x, address);
                }
                None
            }
        },
        None => None,
    };

    if let Some(thread) = result {
        thread.join().unwrap();
    }

    Ok(())
}
