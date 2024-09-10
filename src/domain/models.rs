#[derive(Debug, Default, Clone, Copy)]
pub struct Counter {
    pub value: i32,
}

impl Counter {
    pub fn new(value: i32) -> Self {
        Self { value }
    }
}
