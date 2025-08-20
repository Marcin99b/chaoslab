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
        match command {
            ParsedCommand::Start(name, expose, target) => self.handle_start(name, expose, target),
            ParsedCommand::Stop(name) => self.handle_stop(name),
            ParsedCommand::Slow(name, time) => self.handle_slow(name, time),
            ParsedCommand::Resume(name) => self.handle_resume(name),
        }
    }

    fn handle_start(&self, name: String, expose: String, target: String) -> io::Result<()> {
        let port = expose
            .parse()
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "Invalid port value"))?;

        let r = Redirection::new(name, port, target);
        let t = r.init()?;
        r.set_mode(RedirectionMode::Started);
        self.storage.add(r, t);
        Ok(())
    }

    fn handle_stop(&self, name: String) -> io::Result<()> {
        if let Some(r) = self.storage.find_by_name(&name) {
            r.set_mode(RedirectionMode::Off);
            Ok(())
        } else {
            Err(io::Error::new(
                io::ErrorKind::NotFound,
                "Redirection not found",
            ))
        }
    }

    fn handle_slow(&self, name: String, time: String) -> io::Result<()> {
        let ms = time
            .parse()
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "Invalid ms value"))?;
        if let Some(r) = self.storage.find_by_name(&name) {
            r.set_mode(RedirectionMode::Slowed(ms));
            Ok(())
        } else {
            Err(io::Error::new(
                io::ErrorKind::NotFound,
                "Redirection not found",
            ))
        }
    }

    fn handle_resume(&self, name: String) -> io::Result<()> {
        if let Some(r) = self.storage.find_by_name(&name) {
            r.set_mode(RedirectionMode::Started);
            Ok(())
        } else {
            Err(io::Error::new(
                io::ErrorKind::NotFound,
                "Redirection not found",
            ))
        }
    }
}
