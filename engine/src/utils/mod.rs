use wgpu::{Buffer, util::DeviceExt, BufferUsages};

use crate::Engine;

pub mod initialization;
pub mod bgls;
pub mod shaders;
pub mod materials;

impl Engine {
    #[inline(always)]
    pub fn new_buffer(&self, contents: &[u8], usage: BufferUsages) -> Buffer {
        self.device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: None,
                contents,
                usage
            }
        )
    }
}