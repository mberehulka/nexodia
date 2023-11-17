use engine::Object;

mod main;  pub use main::*;

pub struct Character {
    pub main: Object<crate::shaders::character::main::Shader>,
    pub dir_light: Object<crate::shaders::character::dir_light::Shader>
}