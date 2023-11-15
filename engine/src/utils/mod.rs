use std::sync::{Arc, Mutex, MutexGuard};
use wgpu::{Buffer, util::DeviceExt, BufferUsages};
use winit::dpi::PhysicalPosition;

use crate::Engine;

mod color;  pub use color::*;

pub mod initialization;
pub mod bgls;
pub mod shader;
pub mod pressed_keys;
pub mod animations;
pub mod id;

impl Engine {
    pub fn new_buffer(&self, contents: &[u8], usage: BufferUsages) -> Buffer {
        self.device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: None,
                contents,
                usage
            }
        )
    }
    pub fn center_cursor(&self) {
        let s = self.window.inner_size();
        self.window.set_cursor_position(PhysicalPosition {
            x: s.width/2,
            y: s.height/2
        }).ok();
    }
}

#[derive(Default, Clone)]
pub struct AM<T: Clone>(Arc<Mutex<T>>);
impl<T: Clone> AM<T> {
    #[inline(always)]
    pub fn new(t: T) -> Self {
        Self(Arc::new(Mutex::new(t)))
    }
    #[inline(always)]
    pub fn get(&self) -> T {
        self.0.lock().unwrap().clone()
    }
    #[inline(always)]
    pub fn get_mut<'s>(&'s self) -> MutexGuard<'s, T> {
        self.0.lock().unwrap()
    }
    #[inline(always)]
    pub fn set(&self, t: T) {
        *self.0.lock().unwrap() = t
    }
}