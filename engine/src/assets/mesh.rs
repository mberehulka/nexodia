use std::{path::Path, sync::Arc};
use wgpu::util::DeviceExt;

use crate::{vertex::{VertexType, self}, Engine, Reader};

#[derive(Clone)]
pub struct Mesh {
    pub vertex_type: Arc<VertexType>,
    pub vertices_buffer: Arc<wgpu::Buffer>,
    pub vertices_len: u32
}
impl Engine {
    pub fn load_mesh(&self, path: impl AsRef<Path>) -> Mesh {
        let mut r = Reader::new(path);
        assert!(r.read_byte() == b'M');
        let positions = r.read_vec_f32()
            .chunks(3)
            .into_iter()
            .map(|v|[v[0], v[1], v[2]])
            .collect::<Vec<_>>();
        let indices = r.read_vec_u32_compact();
        let vertex_type = VertexType::P;
        let vertices = indices
            .into_iter()
            .map(|i|match vertex_type {
                VertexType::P => vertex::P {
                    position: positions[i as usize]
                }
            })
            .collect::<Vec<_>>();
        let vertices_buffer = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX
        });
        Mesh {
            vertex_type: vertex_type.into(),
            vertices_buffer: vertices_buffer.into(),
            vertices_len: vertices.len() as u32
        }
    }
}