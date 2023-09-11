use wgpu::{Buffer, util::DeviceExt, BufferUsages};
use winit::dpi::PhysicalPosition;

use crate::Engine;

pub mod initialization;
pub mod bgls;
pub mod shaders;
pub mod materials;
pub mod pressed_keys;

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
        }).unwrap()
    }
}