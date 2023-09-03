/// Basic Fixed Object with vertex::P

use std::sync::Arc;

use engine::{Engine, Material as _, DefaultMaterialBuffer, vertex::{self, Vertex}, ObjectRender};

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
    type Vertex = vertex::p::Vertex;
    type InstanceBinding = ();
    fn pipeline(&self) -> &wgpu::RenderPipeline {
        &self.pipeline
    }
    fn new(e: &'static Engine) -> Self {
        Self {
            pipeline: Arc::new(Self::default_pipeline(
                e,
                wgpu::include_wgsl!("./shader.wgsl").into(),
                &[
                    Self::Vertex::LAYOUT
                ],
                &[
                    &e.camera.bgl,
                    &Material::bind_group_layout(&e.device)
                ]
            ))
        }
    }
}
impl ObjectRender for Shader {}