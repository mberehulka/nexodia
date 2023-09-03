use crate::{Mesh, Engine, Shader, MaterialBuffer};

pub struct Object<S: Shader> {
    pub material_buffer: S::MaterialBuffer,
    pub mesh: Mesh<S::Vertex>
}
impl Engine {
    pub fn new_object<S: Shader>(&self, mesh: Mesh<S::Vertex>, material: S::Material) -> Object<S> {
        Object {
            material_buffer: S::MaterialBuffer::new(self, material),
            mesh
        }
    }
}