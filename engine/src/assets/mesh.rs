use std::{path::Path, sync::Arc, time::Instant, marker::PhantomData};
use compiler::Skeleton;
use wgpu::util::DeviceExt;

use crate::{Engine, Vertex, decode};

#[derive(Clone)]
pub struct Mesh<V: Vertex> {
    vertex_type: PhantomData<V>,
    pub vertices_buffer: Arc<wgpu::Buffer>,
    pub vertices_len: u32,
    pub skeleton: Option<Skeleton>
}
impl Engine {
    pub fn load_mesh<V: Vertex>(&self, path: impl AsRef<Path>) -> Mesh<V> {
        let start = Instant::now();

        let mesh: compiler::Mesh = decode(&path);
        
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

        info!("Mesh '{}' loaded in: {}ms", path.as_ref().display(), (Instant::now() - start).as_millis());
        
        Mesh {
            vertex_type: PhantomData::default(),
            vertices_buffer: vertices_buffer.into(),
            vertices_len,
            skeleton: mesh.skeleton.into()
        }
    }
}