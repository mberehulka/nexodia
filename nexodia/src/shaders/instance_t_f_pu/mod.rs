use engine::{vertex::*, InstanceBinding, InstancesRenderer, Texture, utils::materials::StaticMaterialBuffer, Engine};

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Zeroable, bytemuck::Pod)]
pub struct Instance {
    pub translation: [f32;3],
    pub scale: [f32;3],
    pub texture_id: u32
}
impl InstanceBinding for Instance {}

pub struct Material(pub Vec<Texture>);
impl engine::Material for Material {
    type MaterialBuffer = StaticMaterialBuffer;
    fn create_buffer(&self, e: &'static Engine) -> StaticMaterialBuffer {
        StaticMaterialBuffer(
            e.device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: None,
                layout: &engine::utils::bgls::texture_array(&e.device, self.0.len()as u32),
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureViewArray(
                            &self.0.iter().map(|v|v.view.as_ref()).collect::<Vec<_>>()
                        )
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::SamplerArray(
                            &self.0.iter().map(|v|v.sampler.as_ref()).collect::<Vec<_>>()
                        )
                    }
                ]
            })
        )
    }
}

pub struct Shader(wgpu::RenderPipeline);
impl engine::Shader for Shader {
    type Material = Material;
    type Vertex = engine::vertex::pu::Vertex;
    type Instance = Instance;
    fn pipeline(&self) -> &wgpu::RenderPipeline { &self.0 }
    fn new(e: &'static engine::Engine) -> Self {
        Self(engine::utils::shaders::default_pipeline(
            e,
            wgpu::include_wgsl!("./shader.wgsl").into(),
            &[
                Self::Vertex::LAYOUT,
                wgpu::VertexBufferLayout {
                    array_stride: std::mem::size_of::<Instance>() as wgpu::BufferAddress,
                    step_mode: wgpu::VertexStepMode::Instance,
                    attributes: &wgpu::vertex_attr_array![
                        2 => Float32x3, 3 => Float32x3, 4 => Uint32
                    ]
                }
            ],
            &[
                &e.camera.bgl,
                &engine::utils::bgls::texture_array(&e.device, 1)
            ]
        ))
    }
}
impl InstancesRenderer for Shader {}