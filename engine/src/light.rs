use std::sync::Arc;

use math::Vec3;
use wgpu::{Buffer, BufferUsages};

use crate::Engine;

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct LightBinding {
    pub direction: [f32;3]
}

pub struct Light {
    pub direction: Vec3,
    pub buffer: Arc<Buffer>
}
impl Light {
    pub fn new(e: &Engine, direction: Vec3) -> Self {
        Self {
            buffer: e.new_buffer(
                bytemuck::bytes_of(&LightBinding {
                    direction: direction.into()
                }),
                BufferUsages::UNIFORM | BufferUsages::COPY_DST 
            ).into(),
            direction
        }
    }
}