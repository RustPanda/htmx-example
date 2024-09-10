use crate::domain::{models::Counter, repositories::CounterRepository};

pub struct InMemoryCounterRepository {
    counter: Counter,
}

impl InMemoryCounterRepository {
    pub fn new(value: i32) -> Self {
        InMemoryCounterRepository {
            counter: Counter::new(value),
        }
    }
}

impl CounterRepository for InMemoryCounterRepository {
    fn increment(&mut self) {
        self.counter.value += 1;
    }

    fn decrement(&mut self) {
        self.counter.value -= 1;
    }

    fn get_value(&self) -> i32 {
        self.counter.value
    }
}
