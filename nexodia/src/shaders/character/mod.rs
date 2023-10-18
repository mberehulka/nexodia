use engine::{vertex::*, Engine, Animator, Light};
use wgpu::{Device, BindGroupLayout, BindGroup, Buffer, util::DeviceExt};

pub fn bgl(device: &Device) -> BindGroupLayout {
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: None,
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None
                },
                count: None
            },
            wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None
                },
                count: None
            },
            wgpu::BindGroupLayoutEntry {
                binding: 2,
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

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct MaterialBinding {
    pub color: [f32;4]
}
pub struct Material {
    buffer: Buffer,
    bg: BindGroup
}
impl Material {
    pub fn new(e: &Engine, animator: &Animator, light: &Light) -> Self {
        let buffer = e.device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::bytes_of(&MaterialBinding {
                    color: [1.;4]
                }),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST
            }
        );
        Self {
            bg: e.device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: None,
                layout: &bgl(&e.device),
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: buffer.as_entire_binding()
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: animator.buffer.as_entire_binding()
                    },
                    wgpu::BindGroupEntry {
                        binding: 2,
                        resource: light.buffer.as_entire_binding()
                    }
                ]
            }),
            buffer: buffer
        }
    }
    pub fn update(&self, e: &Engine, value: MaterialBinding) {
        e.queue.write_buffer(&self.buffer, 0, bytemuck::bytes_of(&value))
    }
}
impl engine::Material for Material {
    fn bind_group(&self) -> &wgpu::BindGroup { &self.bg }
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