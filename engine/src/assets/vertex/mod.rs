use bytemuck::Pod;

pub mod p;
pub mod pu;
pub mod puj;
pub mod pnj;
pub mod pj;

pub trait Vertex: Default + Pod {
    const ATTRIBUTES: &'static [wgpu::VertexAttribute];
    const LAYOUT: wgpu::VertexBufferLayout<'static> = wgpu::VertexBufferLayout {
        array_stride: std::mem::size_of::<Self>() as wgpu::BufferAddress,
        step_mode: wgpu::VertexStepMode::Vertex,
        attributes: Self::ATTRIBUTES
    };
    fn new(i: usize, positions: &[[f32;3]], uvs: &[[f32;2]], normals: &[[f32;3]], joints: &[[u8;4]], weights: &[[f32;4]]) -> Self;
}