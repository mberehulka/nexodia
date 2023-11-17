use std::ops::{Index, IndexMut};
use bytemuck::Pod;
use wgpu::util::DeviceExt;

use crate::{Shader, Mesh, Engine, Material};

pub struct Instances <S: Shader> {
    pub material: S::Material,
    mesh: Mesh<S::Vertex>,
    instances_buffer: wgpu::Buffer,
    instances_buffer_length: u32,
    instances: Vec<S::Instance>,
    needs_update: bool
}
impl<S: Shader> Instances<S> {
    pub fn update(&mut self, e: &Engine) {
        if !self.needs_update { return }
        if self.instances_buffer_length != self.instances.len() as u32 {
            self.instances_buffer = e.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(&self.instances),
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST
            })
        } else {
            e.queue.write_buffer(&self.instances_buffer, 0, &bytemuck::cast_slice(&self.instances))
        }
        self.instances_buffer_length = self.instances.len() as u32;
        self.needs_update = false
    }
    pub fn push(&mut self, instance: S::Instance) {
        self.needs_update = true;
        self.instances.push(instance)
    }
}
impl Engine {
    pub fn create_instances<S: Shader>(&'static self, mesh: Mesh<S::Vertex>, material: S::Material, instances: Option<Vec<S::Instance>>) -> Instances<S> {
        match instances {
            Some(instances) => Instances {
                material,
                mesh,
                instances_buffer: self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: None,
                    contents: bytemuck::cast_slice(&instances),
                    usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST
                }),
                instances_buffer_length: instances.len()as u32,
                instances,
                needs_update: false
            },
            None => Instances {
                material,
                mesh,
                instances_buffer: self.device.create_buffer(&wgpu::BufferDescriptor {
                    label: None,
                    mapped_at_creation: false,
                    size: 0,
                    usage: wgpu::BufferUsages::VERTEX
                }),
                instances_buffer_length: 0,
                instances: vec![],
                needs_update: false
            }
        }
    }
}

impl<S: Shader> Index<usize> for Instances<S> {
    type Output = S::Instance;
    fn index(&self, index: usize) -> &Self::Output {
        &self.instances[index]
    }
}
impl<S: Shader> IndexMut<usize> for Instances<S> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.needs_update = true;
        &mut self.instances[index]
    }
}

pub trait InstanceBinding: Pod {}
impl InstanceBinding for () {}

pub trait InstancesRenderer: Shader {
    fn render_instances<'r, 's: 'r>(&'s self, render_pass: &mut wgpu::RenderPass<'r>, instances: &'s Instances<Self>) where Self: Sized {
        if instances.instances_buffer_length == 0 { return }
        render_pass.set_pipeline(self.pipeline());
        render_pass.set_vertex_buffer(0, instances.mesh.vertices_buffer.slice(..));
        render_pass.set_vertex_buffer(1, instances.instances_buffer.slice(..));
        instances.material.set(render_pass);
        render_pass.draw(0..instances.mesh.vertices_len, 0..instances.instances_buffer_length);
    }
}