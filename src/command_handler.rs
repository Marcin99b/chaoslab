use std::{
    io,
    sync::{Arc, Mutex},
    thread::JoinHandle,
};

use crate::{engine::Engine, input_param::ParsedCommand, redirection::Redirection};

pub struct CommandHandler {
    threads_ref: Arc<Mutex<Vec<JoinHandle<()>>>>,
    redirections_ref: Arc<Mutex<Vec<Redirection>>>,
}

impl CommandHandler {
    pub fn from_engine(engine: &Engine) -> CommandHandler {
        CommandHandler {
            threads_ref: Arc::clone(&engine.threads),
            redirections_ref: Arc::clone(&engine.redirections),
        }
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
        let r = Redirection::new(
            args.next().unwrap().to_string(),
            args.next().unwrap().parse().unwrap(),
            args.next().unwrap().to_string(),
        );
        let t = r.init().unwrap();
        r.start();
        self.redirections_ref.lock().unwrap().push(r);
        self.threads_ref.lock().unwrap().push(t);
        Ok(())
    }

    fn handle_stop(&self, command: ParsedCommand) -> io::Result<()> {
        let args = &mut command.args.iter();
        let name = args.next().unwrap().to_string();
        let redirs = self.redirections_ref.lock().unwrap();
        let r = redirs.iter().find(|x| x.name == name).unwrap();
        r.stop();
        Ok(())
    }

    fn handle_slow(&self, command: ParsedCommand) -> io::Result<()> {
        let args = &mut command.args.iter();
        let name = args.next().unwrap().to_string();
        let redirs = self.redirections_ref.lock().unwrap();
        let r = redirs.iter().find(|x| x.name == name).unwrap();
        r.slow(args.next().unwrap().parse().unwrap());
        Ok(())
    }
}
