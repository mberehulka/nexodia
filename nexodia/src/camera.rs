use engine::{camera_bgl, Script};
use wgpu::util::DeviceExt;

pub struct Camera {
    pub bind_group: wgpu::BindGroup,
    pub buffer: wgpu::Buffer,
    pub position: [f32;3],
    pub rotation: [f32;2]
}
impl Script for Camera {
    fn setup(e: &'static engine::Engine) -> Self where Self: Sized {
        let buffer = e.device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: None,
                contents: &bytemuck::cast_slice(&[[0.;16]]),
                usage: wgpu::BufferUsages::UNIFORM
            }
        );
        Self {
            bind_group: e.device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: None,
                layout: &camera_bgl(&e.device),
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: buffer.as_entire_binding()
                    }
                ]
            }),
            buffer,
            position: [0.;3],
            rotation: [0.;2]
        }
    }
}
impl engine::Camera for Camera {
    fn bind_group(&self) -> &wgpu::BindGroup {
        &self.bind_group
    }
}