use std::sync::Arc;
use wgpu::{util::DeviceExt, RenderPipeline};

use crate::Engine;

pub trait Material: Send + Sync {
    fn cast_slice(&self) -> Vec<u8>;
    fn shader(&self, e: &'static Engine) -> &'static RenderPipeline;
}

#[derive(Clone)]
pub struct MaterialBuffer {
    pub shader: &'static RenderPipeline,
    pub bind_group: Arc<wgpu::BindGroup>,
    pub buffer: Arc<wgpu::Buffer>
}
impl Engine {
    pub fn new_material_buffer(&'static self, material: Box<dyn Material>) -> MaterialBuffer {
        let buffer = self.device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: None,
                contents: &material.cast_slice(),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST
            }
        );
        MaterialBuffer {
            bind_group: Arc::new(self.device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: None,
                layout: &material_bgl(&self.device),
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: buffer.as_entire_binding()
                    }
                ]
            })),
            buffer: Arc::new(buffer),
            shader: material.shader(self)
        }
    }
}

pub fn material_bgl(device: &wgpu::Device) -> wgpu::BindGroupLayout {
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
            }
        ]
    })
}