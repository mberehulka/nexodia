use super::vertex::VertexType;

pub struct Mesh {
    pub vertex_type: VertexType,
    pub vertices_buffer: wgpu::Buffer,
    pub vertices: u32
}