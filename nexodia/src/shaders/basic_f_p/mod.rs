/// Basic Fixed Object with vertex::P

use std::sync::Arc;

use engine::{Engine, Material as _, DefaultMaterialBuffer, vertex::Vertex};

#[repr(C)]
#[derive(Default, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ShaderVertex {
    pub position: [f32;3]
}
impl Vertex for ShaderVertex {
    const ATTRIBUTES: &'static [wgpu::VertexAttribute] = &wgpu::vertex_attr_array![
        0 => Float32x3
    ];
    fn requires(_uv: bool, _normal: bool) -> bool { true }
    fn new(i: usize, positions: &[[f32;3]], _uvs: &[[f32;2]], _normals: &[[f32;3]]) -> Self {
        Self {
            position: positions[i]
        }
    }
}

#[repr(C)]
#[derive(Default, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Material {
    pub color: [f32;4]
}
impl engine::Material for Material {
    fn bytes(&self) -> Vec<u8> {
        bytemuck::bytes_of(self).to_vec()
    }
}

pub struct Shader {
    pipeline: Arc<wgpu::RenderPipeline>
}
impl engine::Shader for Shader {
    type Material = Material;
    type MaterialBuffer = DefaultMaterialBuffer;
    type Vertex = ShaderVertex;
    fn pipeline(&self) -> &wgpu::RenderPipeline {
        &self.pipeline
    }
    fn new(e: &'static Engine) -> Self {
        Self {
            pipeline: Arc::new(Self::default_pipeline(
                e,
                wgpu::include_wgsl!("./shader.wgsl").into(),
                &[
                    ShaderVertex::LAYOUT
                ],
                &[
                    &e.camera.bgl,
                    &Material::bind_group_layout(&e.device)
                ]
            ))
        }
    }
}