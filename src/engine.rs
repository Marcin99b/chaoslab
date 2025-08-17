use crate::{
    command_handler::CommandHandler, command_reveiver::CommandReceiver,
    redirections_storage::RedirectionsStorage,
};
use std::thread::{self, JoinHandle};

#[derive(Debug)]
pub struct Engine {
    pub storage: RedirectionsStorage,
}

impl Engine {
    pub fn new() -> Engine {
        Engine {
            storage: RedirectionsStorage::new(),
        }
    }

    pub fn start(&self, address: String) -> JoinHandle<()> {
        let handler = CommandHandler::from_storage(self.storage.clone());
        thread::spawn(move || {
            println!("starting engine {}", address);
            let reveiver = CommandReceiver::new(address);
            for command in reveiver.listen() {
                handler.handle(command).unwrap();
            }
        })
    }
}
