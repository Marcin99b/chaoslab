use crate::command_handler::CommandHandler;
use crate::command_receiver::CommandReceiver;
use crate::redirections_storage::RedirectionsStorage;
use std::thread::{self, JoinHandle};

#[derive(Debug)]
pub struct Engine {
    pub storage: RedirectionsStorage,
}

impl Engine {
    pub fn new() -> Self {
        Self {
            storage: RedirectionsStorage::new(),
        }
    }

    pub fn start(&self, address: String) -> JoinHandle<()> {
        let handler = CommandHandler::from_storage(self.storage.clone());
        thread::spawn(move || {
            println!("starting engine {}", address);
            let receiver = CommandReceiver::new(address);
            for command in receiver.listen() {
                if let Err(e) = handler.handle(command) {
                    eprintln!("Command handling error: {}", e);
                }
            }
        })
    }
}
