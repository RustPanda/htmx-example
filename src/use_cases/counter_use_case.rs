use std::sync::{Arc, Mutex};

use crate::domain::repositories::CounterRepository;

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

    pub async fn get_value(&self) -> i32 {
        let repo = self.repo.lock().unwrap();
        repo.get_value()
    }
}
