use super::models::Counter;

pub trait CounterRepository {
    fn increment(&mut self);
    fn decrement(&mut self);
    fn get(&self) -> Counter;
}
