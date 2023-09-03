use wgpu::{Buffer, util::DeviceExt, BufferUsages, BindGroup};

use crate::{Engine, Texture, Material};

pub mod initialization;
pub mod bgls;
pub mod shaders;

impl Engine {
    #[inline(always)]
    pub fn new_buffer(&self, contents: &[u8], usage: BufferUsages) -> Buffer {
        self.device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: None,
                contents,
                usage
            }
        )
    }
}

#[inline(always)]
pub fn material_bind_group_with_texture<M: Material>(e: &Engine, buffer: &Buffer, texture: &Texture) -> BindGroup {
    e.device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: None,
        layout: &M::bind_group_layout(&e.device),
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding()
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: wgpu::BindingResource::TextureView(&texture.view)
            },
            wgpu::BindGroupEntry {
                binding: 2,
                resource: wgpu::BindingResource::Sampler(&texture.sampler)
            }
        ]
    })
}