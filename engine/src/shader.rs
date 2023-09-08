use wgpu::RenderPipeline;

use crate::{Material, vertex::Vertex, Engine, InstanceBinding};

pub trait Shader {
    type Material: Material;
    type Vertex: Vertex;
    type Instance: InstanceBinding;
    fn pipeline(&self) -> &RenderPipeline;
    fn new(e: &'static Engine) -> Self where Self: Sized;
}