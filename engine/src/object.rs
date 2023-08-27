use crate::{Mesh, Engine, Shader, MaterialBuffer, Instances};

pub struct Object<S: Shader> {
    pub material_buffer: S::MaterialBuffer,
    pub mesh: Mesh<S::Vertex>,
    pub instances: Instances
}
impl<S: Shader> Object<S> {
    pub fn set_material(self, e: &Engine, material: S::Material) -> Self {
        self.material_buffer.set(e, material);
        self
    }
}
impl Engine {
    pub fn new_object<S: Shader>(&self, mesh: Mesh<S::Vertex>, material: S::Material) -> Object<S> {
        Object {
            material_buffer: S::MaterialBuffer::new(self, material),
            mesh,
            instances: Default::default()
        }
    }
}