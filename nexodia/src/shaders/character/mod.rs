use std::num::NonZeroU32;

use engine::{vertex::*, InstanceBinding, InstancesRenderer, Texture, utils::materials::StaticMaterialBuffer, Engine, Animator};
use wgpu::{Device, BindGroupLayout, ShaderModuleDescriptor, ShaderSource};

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Zeroable, bytemuck::Pod)]
pub struct Instance {
    pub translation: [f32;3],
    pub scale: [f32;3],
    pub texture_id: u32
}
impl InstanceBinding for Instance {}

pub fn bind_group_layout(device: &Device, textures: u32) -> BindGroupLayout {
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: None,
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None
                },
                count: None
            },
            wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    view_dimension: wgpu::TextureViewDimension::D2,
                    multisampled: false,
                },
                count: NonZeroU32::new(textures)
            },
            wgpu::BindGroupLayoutEntry {
                binding: 2,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                count: NonZeroU32::new(textures),
            }
        ]
    })
}

pub struct Material {
    pub animator: Animator,
    pub textures: Vec<Texture>
}
impl Material {
    pub fn new(animator: Animator, textures: Vec<Texture>) -> Self {
        Self {
            animator,
            textures
        }
    }
    pub fn create_shader(&self, e: &'static engine::Engine) -> Shader {
        Shader(engine::utils::shaders::default_pipeline(
            e,
            ShaderModuleDescriptor {
                label: None,
                source: ShaderSource::Wgsl(include_str!("./shader.wgsl").into())
            },
            &[
                engine::vertex::puj::Vertex::LAYOUT,
                wgpu::VertexBufferLayout {
                    array_stride: std::mem::size_of::<Instance>() as wgpu::BufferAddress,
                    step_mode: wgpu::VertexStepMode::Instance,
                    attributes: &wgpu::vertex_attr_array![
                        4 => Float32x3, 5 => Float32x3, 6 => Uint32
                    ]
                }
            ],
            &[
                &e.camera.bgl,
                &bind_group_layout(&e.device, self.textures.len()as u32)
            ]
        ))
    }
}
impl engine::Material for Material {
    type MaterialBuffer = StaticMaterialBuffer;
    fn create_buffer(&self, e: &'static Engine) -> StaticMaterialBuffer {
        StaticMaterialBuffer(
            e.device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: None,
                layout: &bind_group_layout(&e.device, self.textures.len()as u32),
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: self.animator.buffer.as_entire_binding()
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::TextureViewArray(
                            &self.textures.iter().map(|v|v.view.as_ref()).collect::<Vec<_>>()
                        )
                    },
                    wgpu::BindGroupEntry {
                        binding: 2,
                        resource: wgpu::BindingResource::SamplerArray(
                            &self.textures.iter().map(|v|v.sampler.as_ref()).collect::<Vec<_>>()
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
    type Vertex = engine::vertex::puj::Vertex;
    type Instance = Instance;
    fn pipeline(&self) -> &wgpu::RenderPipeline { &self.0 }
    fn new(_e: &'static engine::Engine) -> Self {
        unreachable!()
    }
}
impl InstancesRenderer for Shader {}