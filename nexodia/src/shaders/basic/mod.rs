use engine::{Texture, Engine, vertex::*, utils::bgls};
use wgpu::BindGroup;

pub struct Material(BindGroup);
impl Material {
    pub fn new(e: &Engine, texture: Texture) -> Self {
        Material(e.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &bgls::texture(&e.device),
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&texture.view)
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&texture.sampler)
                }
            ]
        }))
    }
}
impl engine::Material for Material {
    fn bind_group(&self) -> &wgpu::BindGroup { &self.0 }
}

shader!(
    Material, engine::vertex::pu::Vertex, (),
    [],
    [engine::utils::bgls::texture]
);
impl engine::ObjectRenderer for Shader {}