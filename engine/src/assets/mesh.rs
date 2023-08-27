use std::{path::Path, sync::Arc, time::Instant, marker::PhantomData};
use wgpu::util::DeviceExt;

use crate::{vertex::Vertex, Engine, Reader};

#[derive(Clone)]
pub struct Mesh<V: Vertex> {
    vertex_type: PhantomData<V>,
    pub vertices_buffer: Arc<wgpu::Buffer>,
    pub vertices_len: u32
}
impl Engine {
    pub fn load_mesh<V: Vertex>(&self, path: impl AsRef<Path>) -> Mesh<V> {
        let start = Instant::now();
        
        let mut r = Reader::new(path.as_ref());
        assert!(r.read_byte() == b'M');

        let positions = r.read_vec_f32()
            .chunks(3)
            .into_iter()
            .map(|v|[v[0], v[1], v[2]])
            .collect::<Vec<_>>();

        let has_uvs = r.read_byte() == 1;
        let uvs = if has_uvs {
            r.read_vec_f32()
                .chunks(2)
                .into_iter()
                .map(|v|[v[0], v[1]])
                .collect::<Vec<_>>()
        } else { vec![] };

        let has_normals = r.read_byte() == 1;
        let normals = if has_normals {
            r.read_vec_f32()
                .chunks(3)
                .into_iter()
                .map(|v|[v[0], v[1], v[2]])
                .collect::<Vec<_>>()
        } else { vec![] };

        let indices = r.read_vec_u32_compact();
        let vertices_len = indices.len() as u32;

        assert!(V::requires(has_uvs, has_normals), "Mesh '{}' is not compatible with this Vertex", path.as_ref().display());
        
        let mut contents = Vec::with_capacity(indices.len() * bytemuck::bytes_of(&V::default()).len());
        for i in indices {
            contents.append(&mut bytemuck::bytes_of(&V::new(i as usize, &positions, &uvs, &normals)).to_vec())
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
            vertices_len
        }
    }
}