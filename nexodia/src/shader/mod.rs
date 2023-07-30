use wgpu::{RenderPipeline, Device, TextureFormat};

pub mod basic_f_p;

pub enum Material {
    BasicFP(basic_f_p::Material)
}

pub struct Shaders {
    basic_f_p: RenderPipeline
}
impl Shaders {
    pub fn new(
        device: &Device,
        surface_texture_format: TextureFormat
    ) -> Self {
        Self {
            basic_f_p: basic_f_p::new(device, surface_texture_format)
        }
    }
}