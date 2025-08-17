use crate::input_param::ParsedCommand;
use crate::redirection::{Redirection, RedirectionMode};
use crate::redirections_storage::RedirectionsStorage;
use std::io;

pub struct CommandHandler {
    storage: RedirectionsStorage,
}

impl CommandHandler {
    pub fn from_storage(storage: RedirectionsStorage) -> Self {
        Self { storage }
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
        let name = args
            .next()
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Missing name argument"))?
            .to_string();
        let port = args
            .next()
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Missing port argument"))?
            .parse()
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "Invalid port value"))?;
        let target = args
            .next()
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Missing target argument"))?
            .to_string();
        let r = Redirection::new(name, port, target);
        let t = r.init()?;
        r.set_mode(RedirectionMode::Started);
        self.storage.add(r, t);
        Ok(())
    }

    fn handle_stop(&self, command: ParsedCommand) -> io::Result<()> {
        let mut args = command.args.iter();
        let name = args
            .next()
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Missing name argument"))?;
        if let Some(r) = self.storage.find_by_name(name) {
            r.set_mode(RedirectionMode::Off);
            Ok(())
        } else {
            Err(io::Error::new(
                io::ErrorKind::NotFound,
                "Redirection not found",
            ))
        }
    }

    fn handle_slow(&self, command: ParsedCommand) -> io::Result<()> {
        let mut args = command.args.iter();
        let name = args
            .next()
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Missing name argument"))?;
        let ms = args
            .next()
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Missing ms argument"))?
            .parse()
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "Invalid ms value"))?;
        if let Some(r) = self.storage.find_by_name(name) {
            r.set_mode(RedirectionMode::Slowed(ms));
            Ok(())
        } else {
            Err(io::Error::new(
                io::ErrorKind::NotFound,
                "Redirection not found",
            ))
        }
    }
}
