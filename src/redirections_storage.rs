use crate::redirection::Redirection;
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;

#[derive(Debug, Clone)]
pub struct RedirectionsStorage {
    pub redirections: Arc<Mutex<Vec<Redirection>>>,
    pub threads: Arc<Mutex<Vec<JoinHandle<()>>>>,
}

impl RedirectionsStorage {
    pub fn new() -> Self {
        Self {
            redirections: Arc::new(Mutex::new(Vec::new())),
            threads: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn find_by_name(&self, name: &str) -> Option<Redirection> {
        let redirs = self.redirections.lock().unwrap();
        redirs.iter().find(|r| r.name.to_string() == name).cloned()
    }

    pub fn add_redirection(&self, redirection: Redirection, handle: JoinHandle<()>) {
        self.redirections.lock().unwrap().push(redirection);
        self.threads.lock().unwrap().push(handle);
    }
}
