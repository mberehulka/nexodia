use wgpu::RenderPass;

use crate::{Mesh, MaterialBuffer, Engine, Material};

#[derive(Clone)]
pub struct Object {
    pub mesh: Mesh,
    pub material: MaterialBuffer
}
impl Engine {
    pub fn new_object(&'static self, mesh: Mesh, material: Box<dyn Material>) -> Object {
        Object {
            mesh,
            material: self.new_material_buffer(material)
        }
    }
    pub fn render_object<'r, 'o: 'r>(&'static self, render_pass: &mut RenderPass<'r>, object: &'o Object) {
        render_pass.set_pipeline(object.material.shader);
        render_pass.set_bind_group(1, &object.material.bind_group, &[]);
        render_pass.set_vertex_buffer(0, object.mesh.vertices_buffer.slice(..));
        render_pass.draw(0..object.mesh.vertices_len, 0..1);
    }
}