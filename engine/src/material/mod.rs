use wgpu::{BindGroup, Buffer};

mod default;  pub use default::*;

use crate::Engine;

pub trait Material {
    fn bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
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
    fn bytes(&self) -> Vec<u8>;
}

pub trait MaterialBuffer<M: Material> {
    fn bind_group(&self) -> &BindGroup;
    fn buffer(&self) -> &Buffer;
    fn new(e: &Engine, material: M) -> Self;
    fn update(&self, _e: &Engine, _material: M) {}
    fn set(&self, e: &Engine, material: M) {
        e.queue.write_buffer(self.buffer(), 0, &material.bytes());
        self.update(e, material);
    }
}