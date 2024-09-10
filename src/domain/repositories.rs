pub trait CounterRepository {
    fn increment(&mut self);
    fn decrement(&mut self);
    fn get_value(&self) -> i32;
}
