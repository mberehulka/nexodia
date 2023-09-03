use std::ops::{Index, IndexMut};

use bytemuck::Pod;
use math::{Quaternion, Vec3};
use wgpu::util::DeviceExt;

use crate::{Shader, Mesh, Engine};

pub struct Instances <S: Shader> {
    mesh: Mesh<S::Vertex>,
    buffer: wgpu::Buffer,
    buffer_length: usize,
    transforms: Vec<S::InstanceBinding>
}
impl<S: Shader> Instances<S> {
    pub fn update(&mut self, e: &Engine) {
        if self.buffer_length != self.transforms.len() {
            self.buffer = e.device.create_buffer_init(
                &wgpu::util::BufferInitDescriptor {
                    label: None,
                    contents: bytemuck::cast_slice(&self.transforms),
                    usage: wgpu::BufferUsages::VERTEX
                }
            )
        } else {
            e.queue.write_buffer(&self.buffer, 0, bytemuck::cast_slice(&self.transforms))
        }
        self.buffer_length = self.transforms.len()
    }
}
impl Engine {
    pub fn new_instances<S: Shader>(&self, mesh: Mesh<S::Vertex>) -> Instances<S> {
        Instances {
            mesh,
            buffer: self.device.create_buffer(&wgpu::BufferDescriptor {
                label: None,
                mapped_at_creation: false,
                size: 0,
                usage: wgpu::BufferUsages::empty()
            }),
            buffer_length: 0,
            transforms: vec![]
        }
    }
}
impl<S: Shader> Index<usize> for Instances<S> {
    type Output = S::InstanceBinding;
    fn index(&self, index: usize) -> &Self::Output {
        &self.transforms[index]
    }
}
impl<S: Shader> IndexMut<usize> for Instances<S> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.transforms[index]
    }
}

pub trait InstanceBinding: Pod {}
impl InstanceBinding for () {}

#[repr(C)]
#[derive(Default, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Transform {
    pub position: Vec3,
    pub scale: Vec3,
    pub rotation: Quaternion
}
impl InstanceBinding for Transform {}