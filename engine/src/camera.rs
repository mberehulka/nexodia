use wgpu::{util::DeviceExt, Device};

use crate::Script;

pub struct DefaultCamera {
    pub bind_group: wgpu::BindGroup
}
impl DefaultCamera {
    pub fn new(device: &Device) -> Self {
        let buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: None,
                contents: &bytemuck::cast_slice(&[[0.;16]]),
                usage: wgpu::BufferUsages::UNIFORM
            }
        );
        Self {
            bind_group: device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: None,
                layout: &camera_bgl(&device),
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: buffer.as_entire_binding()
                    }
                ]
            }),
        }
    }
}
impl Script for DefaultCamera {
    fn setup(e: &'static crate::Engine) -> Self where Self: Sized {
        Self::new(&e.device)
    }
}
impl Camera for DefaultCamera {
    fn bind_group(&self) -> &wgpu::BindGroup {
        &self.bind_group
    }
}

pub trait Camera: Script {
    fn bind_group(&self) -> &wgpu::BindGroup;
}

pub fn camera_bgl(device: &wgpu::Device) -> wgpu::BindGroupLayout {
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: None,
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
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