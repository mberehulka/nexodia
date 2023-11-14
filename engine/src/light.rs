use wgpu::{Buffer, BufferUsages, TextureUsages};
use math::{Vec3, Quaternion, Mat4x4};

use crate::{Engine, DepthTexture};

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct LightBinding {
    pub perspective: [[f32;4];4],
    pub direction: [f32;4]
}

pub struct Light {
    pub direction: Quaternion,
    pub buffer: Buffer,
    pub depth_texture: DepthTexture
}
impl Light {
    pub fn new(
        e: &'static Engine,
        direction: Quaternion,
        (width, height): (u32, u32)
    ) -> Self {
        let s = 10.;
        let dir = direction * Vec3::new(0., 0., 1.);
        let proj = Mat4x4::orthographic(-s, s, -s, s, -s, s);
        let view = Mat4x4::look_at([1.;3].into(), [0.;3].into());
        let perspective = proj * view;
        Self {
            direction,
            buffer: e.new_buffer(
                bytemuck::bytes_of(&LightBinding {
                    perspective: perspective.into(),
                    direction: dir.extend(1.).into()
                }),
                BufferUsages::UNIFORM 
            ),
            depth_texture: DepthTexture::new(&e.device, width, height, TextureUsages::RENDER_ATTACHMENT | TextureUsages::TEXTURE_BINDING)
        }
    }
}