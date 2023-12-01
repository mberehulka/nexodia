use std::ops::{Mul, Sub, Deref, DerefMut};

use engine::Vec2;

pub trait LerpT = Copy + Mul<f32, Output = Self> + Sub<Self, Output = Self>;

pub struct Lerp<T: LerpT> {
    pub value: T,
    pub target: T,
    pub speed: f32
}
impl<T: LerpT> Lerp<T> {
    pub const fn new(value: T, speed: f32) -> Self {
        Self {
            value,
            target: value,
            speed
        }
    }
}
impl<T: LerpT> Deref for Lerp<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}
impl<T: LerpT> DerefMut for Lerp<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl Lerp<Vec2> {
    pub fn lerp(&mut self, amount: f32) {
        self.value.lerp(self.target, self.speed * amount)
    }
}