mod engine;
mod input_param;
mod redirection;

use std::io;

use crate::engine::Engine;

fn main() -> io::Result<()> {
    let result = match std::env::args().nth(0) {
        Some(x) => match x.as_str() {
            "engine" => Some(Engine::new().start("127.0.0.1:9900".to_string())),
            _ => None,
        },
        None => None,
    };

    if let Some(thread) = result {
        thread.join().unwrap();
    }

    Ok(())
}
