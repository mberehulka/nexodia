use engine::{Object, Engine, Texture, Mesh, Shader, Light};

use crate::shaders::basic;

pub struct BasicObject {
    pub main: Object<basic::main::Shader>,
    pub dir_light: Object<basic::dir_light::Shader>
}
impl BasicObject {
    pub fn new(
        e: &'static Engine,
        dir_light: &Light,
        texture: Texture,
        mesh: Mesh<<basic::main::Shader as Shader>::Vertex>
    ) -> Self {
        Self {
            main: e.create_object(basic::main::Material::new(e, dir_light, texture), mesh.clone()),
            dir_light: e.create_object(basic::dir_light::Material::new(e, dir_light), mesh)
        }
    }
}