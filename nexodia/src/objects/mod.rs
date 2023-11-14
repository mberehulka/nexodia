use engine::ScriptHandler;

mod character;  pub use character::*;
mod third_p_c;  pub use third_p_c::*;
mod basic;      pub use basic::*;

pub type Camera = (ScriptHandler, CameraValues);