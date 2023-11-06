#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use winit::{event::VirtualKeyCode, dpi::PhysicalSize};
use engine::{Engine, Script, Frame, Object, ObjectRenderer, Quaternion, Light, decode, Transform};

#[macro_use]
extern crate engine;

mod shaders;  use shaders::*;
mod scripts;  use scripts::*;

pub struct Scene {
    e: &'static Engine,
    _camera: Camera,
    frame: Frame,
    shaders: Shaders,
    scenary: Vec<Object<basic::Shader>>,
    _light: Light,
    main_char: Character
}
impl Scene {
    fn new(e: &'static Engine) -> Self {
        let camera = ThirdPersonCamera::new(e);

        let light = Light::new(e, Quaternion::from_angle_x(0.));
        
        let char_animations = CharacterAnimations::new(e).into();
        let char_mesh = e.load_mesh("assets/male/base/base.bin");
        let main_char = MainCharacter::new(e, char_animations, char_mesh, &light, camera.1.clone());
        
        Self {
            e,
            _camera: camera,
            frame: Frame::new(e, true),
            shaders: Shaders::new(e),
            scenary: vec![
                e.create_object(
                    basic::Material::new(e, e.load_texture("assets/textures/grass.bin")),
                    e.initialize_mesh(
                        decode::<engine::compiler::Mesh>("assets/geometries/cube.bin")
                            .transform(Transform::from_scale(5., 0., 5.))
                    )
                )
            ],
            _light: light,
            main_char
        }
    }
}
impl Script for Scene {
    fn name() -> &'static str { "MainScene" }
    fn on_key_press(&mut self, key: VirtualKeyCode) {
        if let VirtualKeyCode::Escape = key { self.e.exit() }
    }
    fn window_resized(&mut self, _new_size: PhysicalSize<u32>) {
        self.frame.window_resized()
    }
    fn update(&mut self) {
        let mut render_pass = self.frame.new_render_pass(true);
        render_pass.set_bind_group(0, &self.e.camera_buffer.bind_group, &[]);
        self.shaders.character.render_object(&mut render_pass, &self.main_char.1);
        for obj in self.scenary.iter() {
            self.shaders.basic.render_object(&mut render_pass, obj)
        }
    }
    fn render(&mut self) {
        self.frame.render()
    }
}

fn main() {
    let (el, e) = engine::Engine::new();
    e.scripts.add(Scene::new(e)).make_static();
    e.start(el)
}