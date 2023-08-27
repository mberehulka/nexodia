use std::time::Instant;

use engine::{Engine, Shader};

pub mod basic_f_p;
pub mod texture_f_pu;

pub struct Shaders {
    pub basic_f_p: basic_f_p::Shader,
    pub texture_f_pu: texture_f_pu::Shader
}
impl Shaders {
    pub fn new(e: &'static Engine) -> Self {
        let start = Instant::now();
        let s = Self {
            basic_f_p: basic_f_p::Shader::new(e),
            texture_f_pu: texture_f_pu::Shader::new(e)
        };
        info!("Shaders compiled in: {}ms", (Instant::now() - start).as_millis());
        s
    }
}