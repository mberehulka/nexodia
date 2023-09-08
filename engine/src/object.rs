use crate::{Mesh, Engine, Shader, MaterialBuffer, Material};

pub struct Object<S: Shader> {
    pub material_buffer: <S::Material as Material>::MaterialBuffer,
    pub mesh: Mesh<S::Vertex>
}
impl Engine {
    pub fn create_object<S: Shader>(&'static self, mesh: Mesh<S::Vertex>, material: S::Material) -> Object<S> {
        Object {
            material_buffer: material.create_buffer(self),
            mesh
        }
    }
}

pub trait ObjectRenderer: Shader {
    fn render_object<'r, 's: 'r>(&'s self, render_pass: &mut wgpu::RenderPass<'r>, object: &'s Object<Self>) where Self: Sized {
        render_pass.set_pipeline(self.pipeline());
        if let Some(mbg) = object.material_buffer.bind_group() {
            render_pass.set_bind_group(1, mbg, &[])
        }
        render_pass.set_vertex_buffer(0, object.mesh.vertices_buffer.slice(..));
        render_pass.draw(0..object.mesh.vertices_len, 0..1);
    }
}