use std::marker::PhantomData;

use wgpu::{Device, util::DeviceExt};

pub trait BufferType {
    fn bind_group_layout(device: &Device) -> wgpu::BindGroupLayout;
}

pub struct Buffer<T: BufferType> {
    pub bind_group: wgpu::BindGroup,
    pub buffer: wgpu::Buffer,
    pub _t: PhantomData<T>
}
impl<T: BufferType> Buffer<T> {
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
                layout: &T::bind_group_layout(device),
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: buffer.as_entire_binding()
                    }
                ]
            }),
            buffer,
            _t: PhantomData::default()
        }
    }
}