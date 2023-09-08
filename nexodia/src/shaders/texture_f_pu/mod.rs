use engine::{Texture, Engine, vertex::*, utils::{bgls, materials::StaticMaterialBuffer}};

pub struct Material(pub Texture);
impl engine::Material for Material {
    type MaterialBuffer = StaticMaterialBuffer;
    fn create_buffer(&self, e: &'static Engine) -> StaticMaterialBuffer {
        StaticMaterialBuffer(
            e.device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: None,
                layout: &bgls::texture(&e.device),
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(&self.0.view)
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(&self.0.sampler)
                    }
                ]
            })
        )
    }
}

shader!(
    Material, engine::vertex::pu::Vertex, (),
    [],
    [engine::utils::bgls::texture]
);
impl engine::ObjectRenderer for Shader {}