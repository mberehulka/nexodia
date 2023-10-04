use std::sync::Arc;

use math::{Vec3, Quaternion};
use wgpu::{Buffer, BufferUsages};

use crate::Engine;

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct LightBinding {
    pub direction: [f32;4]
}

pub struct Light {
    pub direction: Quaternion,
    pub buffer: Arc<Buffer>
}
impl Light {
    pub fn new(e: &Engine, direction: Quaternion) -> Self {
        let dir = direction * Vec3::new(0., 0., 1.);
        Self {
            buffer: e.new_buffer(
                bytemuck::bytes_of(&LightBinding {
                    direction: dir.extend(1.).into()
                }),
                BufferUsages::UNIFORM | BufferUsages::COPY_DST 
            ).into(),
            direction
        }
    }
}