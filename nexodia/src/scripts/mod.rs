use engine::{ScriptHandler, Object};

use crate::shaders::character::Shader;

mod main_char;  pub use main_char::*;
mod third_p_c;     pub use third_p_c::*;

pub type Camera = (ScriptHandler, CameraValues);
pub type Character = (ScriptHandler, Object<Shader>);