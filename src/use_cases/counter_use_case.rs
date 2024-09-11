use std::sync::{Arc, RwLock};

use tokio::sync::broadcast::{Receiver, Sender};

use crate::domain::repositories::CounterRepository;

#[derive(Clone)]
pub struct CounterUseCase {
    repo: Arc<RwLock<dyn CounterRepository + Send + Sync + 'static>>,
    sender: Sender<i32>,
}

impl CounterUseCase {
    pub fn new(repo: impl CounterRepository + Send + Sync + 'static) -> Self {
        CounterUseCase {
            repo: Arc::new(RwLock::new(repo)),
            sender: Sender::new(5),
        }
    }

    pub async fn increment(&self) {
        let mut repo = self.repo.write().unwrap();
        repo.increment();
        let value = repo.get_value();
        drop(repo);
        let _ = self.sender.send(value);
    }

    pub async fn decrement(&self) {
        let mut repo = self.repo.write().unwrap();
        repo.decrement();
        let value = repo.get_value();
        drop(repo);
        let _ = self.sender.send(value);
    }

    pub async fn get_value(&self) -> i32 {
        let repo = self.repo.read().unwrap();
        let value = repo.get_value();
        drop(repo);
        value
    }

    pub fn subscribe(&self) -> Receiver<i32> {
        self.sender.subscribe()
    }
}
