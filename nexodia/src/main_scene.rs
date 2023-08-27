use engine::{Object, Engine, Script, Shader, Frame};
use winit::event::VirtualKeyCode;

use crate::shaders::*;

pub struct Scene {
    e: &'static Engine,
    shaders: Shaders,
    cube: Object<basic_f_p::Shader>,
    character: Object<texture_f_pu::Shader>
}
impl Script for Scene {
    fn new(e: &'static Engine) -> Self {
        Self {
            e,
            shaders: Shaders::new(e),
            cube: e.new_object(
                e.load_mesh("assets/cube.bin"),
                basic_f_p::Material {
                    color: [0.5;4]
                }
            ),
            character: e.new_object(
                e.load_mesh("assets/mannequin/mannequin.bin"),
                texture_f_pu::Material {
                    texture: e.load_texture("assets/mannequin/textures/Ch36_1001_Diffuse.bin"),
                    color: [1.;4]
                }
            )
        }
    }
    fn on_key_press(&mut self, key: VirtualKeyCode) {
        if let VirtualKeyCode::Escape = key { self.e.exit() }
    }
    fn render(&mut self, frame: &mut Frame) {
        let mut render_pass = frame.new_render_pass(true, true);
        render_pass.set_bind_group(0, &self.e.camera.bind_group, &[]);
        self.shaders.basic_f_p.render_object(&mut render_pass, &self.cube);
        self.shaders.texture_f_pu.render_object(&mut render_pass, &self.character);
    }
}