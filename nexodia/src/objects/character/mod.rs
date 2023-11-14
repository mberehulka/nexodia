mod main;  use engine::{ScriptHandler, Object};
pub use main::*;

pub struct Character {
    pub script: ScriptHandler,
    pub main: Object<crate::shaders::character::main::Shader>,
    pub dir_light: Object<crate::shaders::character::dir_light::Shader>
}