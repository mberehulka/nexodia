use crate::{Mesh, Engine, Shader, Material};

pub struct Object<S: Shader> {
    pub material: S::Material,
    pub mesh: Mesh<S::Vertex>
}
impl Engine {
    pub fn create_object<S: Shader>(&'static self, material: S::Material, mesh: Mesh<S::Vertex>) -> Object<S> {
        Object {
            material,
            mesh
        }
    }
}

pub trait ObjectRenderer: Shader {
    fn render_object<'r, 's: 'r>(&'s self, render_pass: &mut wgpu::RenderPass<'r>, object: &'s Object<Self>) where Self: Sized {
        render_pass.set_pipeline(self.pipeline());
        render_pass.set_vertex_buffer(0, object.mesh.vertices_buffer.slice(..));
        object.material.set(render_pass);
        render_pass.draw(0..object.mesh.vertices_len, 0..1);
    }
}