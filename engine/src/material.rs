use wgpu::BindGroup;

use crate::Engine;

pub trait Material {
    type MaterialBuffer: MaterialBuffer;
    fn create_buffer(&self, e: &'static Engine) -> Self::MaterialBuffer;
}

pub trait MaterialBuffer: Send + Sync {
    fn bind_group(&self) -> Option<&BindGroup>;
}