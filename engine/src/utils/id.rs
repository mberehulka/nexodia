use std::sync::Mutex;

pub type IdT = u128;

pub struct Id {
    id: Mutex<IdT>
}
impl Id {
    pub const fn default() -> Self {
        Self {
            id: Mutex::new(0)
        }
    }
    pub fn next(&self) -> IdT {
        let mut id = self.id.lock().unwrap();
        *id += 1;
        assert!(*id < IdT::MAX);
        *id
    }
}