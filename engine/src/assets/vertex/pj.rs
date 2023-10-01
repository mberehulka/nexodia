#[repr(C)]
#[derive(Default, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32;3],
    pub joints: [u32;4],
    pub weights: [f32;4]
}
impl crate::Vertex for Vertex {
    const ATTRIBUTES: &'static [wgpu::VertexAttribute] = &wgpu::vertex_attr_array![
        0 => Float32x3, 1 => Uint32x4, 2 => Float32x4
    ];
    fn new(i: usize, positions: &[[f32;3]], _uvs: &[[f32;2]], _normals: &[[f32;3]], joints: &[[u8;4]], weights: &[[f32;4]]) -> Self {
        Self {
            position: positions[i],
            joints: [joints[i][0] as u32, joints[i][1] as u32, joints[i][2] as u32, joints[i][3] as u32],
            weights: weights[i]
        }
    }
}