#[repr(C)]
#[derive(Default, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32;3],
    pub uv: [f32;2]
}
impl crate::Vertex for Vertex {
    const ATTRIBUTES: &'static [wgpu::VertexAttribute] = &wgpu::vertex_attr_array![
        0 => Float32x3, 1 => Float32x2
    ];
    fn new(i: usize, positions: &[[f32;3]], uvs: &[[f32;2]], _normals: &[[f32;3]], _joints: &[[u8;4]], _weights: &[[f32;4]]) -> Self {
        Self {
            position: positions[i],
            uv: uvs[i]
        }
    }
}