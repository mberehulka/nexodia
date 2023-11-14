use std::{path::Path, marker::PhantomData, sync::Arc};
use compiler::Skeleton;
use math::Transform;
use wgpu::util::DeviceExt;

use crate::{Engine, Vertex, decode};

#[derive(Clone)]
pub struct Mesh<V: Vertex> {
    vertex_type: PhantomData<V>,
    pub vertices_buffer: Arc<wgpu::Buffer>,
    pub vertices_len: u32,
    pub skeleton: Option<Arc<Skeleton>>
}
impl Engine {
    pub fn load_mesh<V: Vertex>(&self, path: impl AsRef<Path>) -> Mesh<V> {
        self.initialize_mesh(decode(&path))
    }
    pub fn initialize_mesh<V: Vertex>(&self, mesh: compiler::Mesh) -> Mesh<V> {
        let vertices_len = mesh.indices.len() as u32;
        let mut contents = Vec::with_capacity(mesh.indices.len() * bytemuck::bytes_of(&V::default()).len());
        for i in mesh.indices {
            contents.append(&mut bytemuck::bytes_of(&V::new(i as usize, &mesh.positions, &mesh.uvs, &mesh.normals, &mesh.joints, &mesh.weights)).to_vec())
        }

        let vertices_buffer = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: &contents,
            usage: wgpu::BufferUsages::VERTEX
        });

        Mesh {
            vertex_type: PhantomData::default(),
            vertices_buffer: vertices_buffer.into(),
            vertices_len,
            skeleton: if let Some(skeleton) = mesh.skeleton { Some(skeleton.into()) } else { None }
        }
    }
    pub fn transformed_mesh<V: Vertex>(&self, path: impl AsRef<Path>, transform: Transform) -> Mesh<V> {
        self.initialize_mesh(decode::<compiler::Mesh>(&path).transform(transform))
    }
}