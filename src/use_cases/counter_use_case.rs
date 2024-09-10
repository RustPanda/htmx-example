use std::sync::{Arc, Mutex};

use crate::domain::{models::Counter, repositories::CounterRepository};

#[derive(Clone)]
pub struct CounterUseCase {
    repo: Arc<Mutex<dyn CounterRepository + Send>>,
}

impl CounterUseCase {
    pub fn new(repo: impl CounterRepository + Send + 'static) -> Self {
        CounterUseCase {
            repo: Arc::new(Mutex::new(repo)),
        }
    }

    pub async fn increment(&self) {
        let mut repo = self.repo.lock().unwrap();
        repo.increment();
    }

    pub async fn decrement(&self) {
        let mut repo = self.repo.lock().unwrap();
        repo.decrement();
    }

    pub async fn get(&self) -> Counter {
        let repo = self.repo.lock().unwrap();
        repo.get()
    }
}
