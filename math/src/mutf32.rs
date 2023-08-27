use std::{sync::atomic::AtomicU32, mem::transmute};

#[derive(Default)]
pub struct MutF32(AtomicU32);
impl MutF32 {
    #[inline(always)]
    pub const fn new(v: f32) -> Self {
        Self(AtomicU32::new(unsafe{transmute(v)}))
    }
    #[inline(always)]
    pub fn set(&self, v: f32) {
        self.0.store(unsafe{transmute(v)}, std::sync::atomic::Ordering::Relaxed)
    }
    #[inline(always)]
    pub fn get(&self) -> f32 {
        unsafe{transmute(self.0.load(std::sync::atomic::Ordering::Relaxed))}
    }
    #[inline(always)]
    pub fn mul(&self, v: f32) {
        self.set(self.get()*v)
    }
    #[inline(always)]
    pub fn div(&self, v: f32) {
        self.set(self.get()/v)
    }
    #[inline(always)]
    pub fn add(&self, v: f32) {
        self.set(self.get()+v)
    }
    #[inline(always)]
    pub fn sub(&self, v: f32) {
        self.set(self.get()-v)
    }
}
impl Into<MutF32> for f32 {
    fn into(self) -> MutF32 {
        MutF32::new(self)
    }
}