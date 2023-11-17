use std::sync::Mutex;

pub type Id = u128;

pub struct IdHandler {
    id: Mutex<Id>
}
impl IdHandler {
    pub const fn default() -> Self {
        Self {
            id: Mutex::new(0)
        }
    }
    pub fn next(&self) -> Id {
        let mut id = self.id.lock().unwrap();
        *id += 1;
        assert!(*id < Id::MAX);
        *id
    }
}