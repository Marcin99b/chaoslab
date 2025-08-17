use std::{
    sync::{Arc, Mutex},
    thread::{self, JoinHandle},
};

use crate::{
    command_handler::CommandHandler, command_reveiver::CommandReceiver, redirection::Redirection,
};

#[derive(Debug)]
pub struct Engine {
    pub threads: Arc<Mutex<Vec<JoinHandle<()>>>>,
    pub redirections: Arc<Mutex<Vec<Redirection>>>,
}

impl Engine {
    pub fn new() -> Engine {
        Engine {
            threads: Arc::new(Mutex::new(Vec::new())),
            redirections: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn start(&self, address: String) -> JoinHandle<()> {
        let handler = CommandHandler::from_engine(&self);
        thread::spawn(move || {
            println!("starting engine {}", address);
            let reveiver = CommandReceiver::new(address);
            for command in reveiver.listen() {
                handler.handle(command).unwrap();
            }
        })
    }
}
