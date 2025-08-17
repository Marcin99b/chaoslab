// ...existing code...
mod cli;
mod command_handler;
mod command_receiver;
mod engine;
mod input_param;
mod redirection;
mod redirections_storage;

use std::io;

use crate::engine::Engine;

fn main() -> io::Result<()> {
    let address = "127.0.0.1:9900".to_string();

    let result = match std::env::args().nth(1) {
        Some(arg) => {
            if arg == "engine" {
                Some(Engine::new().start(address))
            } else {
                if arg.len() > 3 {
                    cli::send(arg, address);
                }
                None
            }
        }
        None => None,
    };

    if let Some(thread) = result {
        if let Err(e) = thread.join() {
            eprintln!("Engine thread panicked: {:?}", e);
        }
    }

    Ok(())
}
