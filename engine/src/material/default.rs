use wgpu::{BindGroup, Buffer, util::DeviceExt};

use crate::{Material, MaterialBuffer, Engine};

pub struct DefaultMaterialBuffer {
    pub bind_group: BindGroup,
    pub buffer: Buffer
}
impl<M: Material> MaterialBuffer<M> for DefaultMaterialBuffer {
    fn bind_group(&self) -> &BindGroup {
        &self.bind_group
    }
    fn buffer(&self) -> &Buffer {
        &self.buffer
    }
    fn new(e: &Engine, material: M) -> Self {
        let buffer = e.device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: None,
                contents: &material.bytes(),
                usage: wgpu::BufferUsages::UNIFORM
            }
        );
        DefaultMaterialBuffer {
            bind_group: e.device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: None,
                layout: &M::bind_group_layout(&e.device),
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: buffer.as_entire_binding()
                    }
                ]
            }),
            buffer
        }
    }
}