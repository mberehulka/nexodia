/// Textured Fixed Object with vertex::P

use std::sync::Arc;
use engine::{Engine, Material as _, Texture, vertex::{self, Vertex}, ObjectRender};

#[repr(C)]
#[derive(Default, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct MaterialBinding {
    pub color: [f32;4]
}
pub struct Material {
    pub texture: Texture,
    pub color: [f32;4]
}
impl engine::Material for Material {
    fn bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
        engine::utils::bgls::material_with_texture(device)
    }
    fn bytes(&self) -> Vec<u8> {
        bytemuck::bytes_of(&MaterialBinding { color: self.color }).to_vec()
    }
}

pub struct MaterialBuffer {
    bind_group: Arc<wgpu::BindGroup>,
    buffer: Arc<wgpu::Buffer>
}
impl engine::MaterialBuffer<Material> for MaterialBuffer {
    fn bind_group(&self) -> &wgpu::BindGroup { &self.bind_group }
    fn buffer(&self) -> &wgpu::Buffer { &self.buffer }
    fn new(e: &Engine, material: Material) -> Self {
        let buffer = e.new_buffer(&material.bytes(), wgpu::BufferUsages::UNIFORM);
        Self {
            bind_group: engine::utils::material_bind_group_with_texture::<Material>(e, &buffer, &material.texture).into(),
            buffer: buffer.into()
        }
    }
}

pub struct Shader(wgpu::RenderPipeline);
impl engine::Shader for Shader {
    type Material = Material;
    type MaterialBuffer = MaterialBuffer;
    type Vertex = vertex::pu::Vertex;
    type InstanceBinding = ();
    fn pipeline(&self) -> &wgpu::RenderPipeline { &self.0 }
    fn new(e: &'static Engine) -> Self {
        Self(Self::default_pipeline(
            e,
            wgpu::include_wgsl!("./shader.wgsl").into(),
            &[
                Self::Vertex::LAYOUT
            ],
            &[
                &e.camera.bgl,
                &Material::bind_group_layout(&e.device)
            ]
        ))
    }
}
impl ObjectRender for Shader {}