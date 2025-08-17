use crate::{
    input_param::ParsedCommand, redirection::Redirection, redirections_storage::RedirectionsStorage,
};
use std::io;

pub struct CommandHandler {
    storage: RedirectionsStorage,
}

impl CommandHandler {
    pub fn from_storage(storage: RedirectionsStorage) -> CommandHandler {
        CommandHandler { storage }
    }

    pub fn handle(&self, command: ParsedCommand) -> io::Result<()> {
        match command.name.as_str() {
            "start" => self.handle_start(command),
            "stop" => self.handle_stop(command),
            "slow" => self.handle_slow(command),
            _ => Err(io::Error::new(
                io::ErrorKind::Other,
                format!("Unknown command: {}", command.name),
            )),
        }
    }

    fn handle_start(&self, command: ParsedCommand) -> io::Result<()> {
        let mut args = command.args.iter();
        let name = args.next().unwrap().to_string();
        let port = args.next().unwrap().parse().unwrap();
        let target = args.next().unwrap().to_string();
        let r = Redirection::new(name, port, target);
        let t = r.init().unwrap();
        r.start();
        self.storage.add_redirection(r, t);
        Ok(())
    }

    fn handle_stop(&self, command: ParsedCommand) -> io::Result<()> {
        let args = &mut command.args.iter();
        let name = args.next().unwrap();
        if let Some(r) = self.storage.find_by_name(name) {
            r.stop();
            Ok(())
        } else {
            Err(io::Error::new(
                io::ErrorKind::NotFound,
                "Redirection not found",
            ))
        }
    }

    fn handle_slow(&self, command: ParsedCommand) -> io::Result<()> {
        let args = &mut command.args.iter();
        let name = args.next().unwrap();
        let ms = args.next().unwrap().parse().unwrap();
        if let Some(r) = self.storage.find_by_name(name) {
            r.slow(ms);
            Ok(())
        } else {
            Err(io::Error::new(
                io::ErrorKind::NotFound,
                "Redirection not found",
            ))
        }
    }
}
