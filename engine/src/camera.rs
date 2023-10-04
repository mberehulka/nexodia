use wgpu::{util::DeviceExt, Device};

use crate::Engine;

#[repr(C)]
#[derive(Default, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraBinding {
    pub matrix: [[f32;4];4],
    pub position: [f32;4]
}

pub struct Camera {
    buffer: wgpu::Buffer,
    pub bind_group: wgpu::BindGroup,
    pub bgl: wgpu::BindGroupLayout
}
impl Camera {
    pub fn new(device: &Device) -> Self {
        let buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: None,
                contents: &bytemuck::cast_slice(&[CameraBinding::default()]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST
            }
        );
        let bgl = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
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
        });
        Self {
            bind_group: device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: None,
                layout: &bgl,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: buffer.as_entire_binding()
                    }
                ]
            }),
            buffer,
            bgl
        }
    }
}
impl Engine {
    pub fn update_camera_buffer(&self, buffer: CameraBinding) {
        self.queue.write_buffer(&self.camera.buffer, 0, bytemuck::cast_slice(&[buffer]))
    }
}