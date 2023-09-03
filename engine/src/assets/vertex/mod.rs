use bytemuck::Pod;

pub mod pu;
pub mod p;

pub trait Vertex: Default + Pod {
    const ATTRIBUTES: &'static [wgpu::VertexAttribute];
    const LAYOUT: wgpu::VertexBufferLayout<'static> = wgpu::VertexBufferLayout {
        array_stride: std::mem::size_of::<Self>() as wgpu::BufferAddress,
        step_mode: wgpu::VertexStepMode::Vertex,
        attributes: Self::ATTRIBUTES
    };
    fn requires(uv: bool, normal: bool) -> bool;
    fn new(i: usize, positions: &[[f32;3]], uvs: &[[f32;2]], normals: &[[f32;3]]) -> Self;
}