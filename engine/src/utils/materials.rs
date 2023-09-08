use bytemuck::Pod;
use wgpu::{BindGroup, Buffer, util::DeviceExt, BindGroupLayout};

use crate::{MaterialBuffer, Engine, Material};

pub struct StaticMaterialBuffer(pub BindGroup);
impl MaterialBuffer for StaticMaterialBuffer {
    fn bind_group(&self) -> Option<&BindGroup> {
        Some(&self.0)
    }
}

pub struct UniformBuffer(BindGroup, Buffer);
impl UniformBuffer {
    pub fn new(e: &Engine, layout: &BindGroupLayout, v: &impl Pod) -> Self {
        let buffer = e.device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::bytes_of(v),
                usage: wgpu::BufferUsages::UNIFORM
            }
        );
        Self (
            e.device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: None,
                layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: buffer.as_entire_binding()
                    }
                ]
            }),
            buffer
        )
    }
}
impl MaterialBuffer for UniformBuffer {
    fn bind_group(&self) -> Option<&BindGroup> {
        Some(&self.0)
    }
}
#[macro_export]
macro_rules! uniform_buffer {
    ($($prop: ident: $type: ty),*) => {
        #[repr(C)]
        #[derive(Clone, Copy, Default, bytemuck::Zeroable, bytemuck::Pod)]
        pub struct Material {
            $( pub $prop: $type ),*
        }
        impl engine::Material for Material {
            type MaterialBuffer = engine::utils::materials::UniformBuffer;
            fn create_buffer(&self, e: &'static engine::Engine) -> engine::utils::materials::UniformBuffer {
                engine::utils::materials::UniformBuffer::new(e, &engine::utils::bgls::uniform(&e.device), self)
            }
        }
    };
}
pub use uniform_buffer;

impl Material for () {
    type MaterialBuffer = ();
    fn create_buffer(&self, _e: &'static Engine) -> Self::MaterialBuffer {
        ()
    }
}
impl MaterialBuffer for () {
    fn bind_group(&self) -> Option<&BindGroup> {
        None
    }
}