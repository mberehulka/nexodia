use engine::{vertex::*, Engine, Animator, Light};
use wgpu::{Device, BindGroupLayout, BindGroup};

pub fn bgl(device: &Device) -> BindGroupLayout {
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
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None
                },
                count: None
            }
        ]
    })
}

pub struct Material(BindGroup);
impl Material {
    pub fn new(e: &Engine, animator: &Animator, light: &Light) -> Self {
        Self(e.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &bgl(&e.device),
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: animator.buffer.as_entire_binding()
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: light.buffer.as_entire_binding()
                }
            ]
        }))
    }
}
impl engine::Material for Material {
    fn bind_group(&self) -> &wgpu::BindGroup { &self.0 }
}

shader!(
    Material,
    engine::vertex::pnj::Vertex,
    (),
    [],
    [
        bgl
    ]
);
impl engine::ObjectRenderer for Shader {}