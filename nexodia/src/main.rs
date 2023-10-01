#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::time::Instant;
use engine::{Engine, Script, Frame, Object, ObjectRenderer, Quaternion, deg_to_rad, decode, Transform, Vec3, Animator, Light};
use winit::{event::VirtualKeyCode, dpi::PhysicalSize};

#[macro_use]
extern crate engine;

mod camera;   use camera::*;
mod shaders;  use shaders::*;

pub struct Scene {
    e: &'static Engine,
    frame: Frame,
    shaders: Shaders,
    character: Object<character::Shader>,
    character_face: Object<character::Shader>,
    character_animator: Animator,
    scenary: Vec<Object<basic::Shader>>
}
impl Scene {
    fn new(e: &'static Engine) -> Self {
        let character_mesh = e.load_mesh("assets/male/male.bin");
        let mut character_animator = e.load_animations(&character_mesh, "assets/male/animations/");
        character_animator.transform.scale = 2.0.into();
        character_animator.transform.rotation = Quaternion::from_angle_x(deg_to_rad(-90.));

        let light = Light::new(e, Vec3::new(0., 0., 0.));
        
        Self {
            e,
            frame: Frame::new(e, true),
            shaders: Shaders::new(e),
            character: e.create_object(
                character::Material::new(e, &character_animator, &light),
                character_mesh
            ),
            character_face: e.create_object(
                character::Material::new(e, &character_animator, &light),
                e.load_mesh("assets/male/male_face.bin")
            ),
            character_animator,
            scenary: vec![
                e.create_object(
                    basic::Material::new(e, e.load_texture("assets/textures/grass.bin")),
                    e.initialize_mesh(
                        decode::<engine::compiler::Mesh>("assets/geometries/cube.bin")
                            .transform(Transform::new(
                                Vec3::new(0., 0., 0.),
                                Quaternion::default(),
                                Vec3::new(5., 0., 5.)
                            ))
                    )
                )
            ]
        }
    }
}
impl Script for Scene {
    fn on_key_press(&mut self, key: VirtualKeyCode) {
        if let VirtualKeyCode::Escape = key { self.e.exit() }
    }
    fn window_resized(&mut self, _new_size: PhysicalSize<u32>) {
        self.frame.window_resized()
    }
    fn update(&mut self) {
        if self.e.pressed_keys["C"] {
            self.character_animator.set_animation(1)
        } else {
            self.character_animator.set_animation(0)
        }
        self.character_animator.update(self.e);

        let mut render_pass = self.frame.new_render_pass(true);
        render_pass.set_bind_group(0, &self.e.camera.bind_group, &[]);
        self.shaders.character.render_object(&mut render_pass, &self.character);
        self.shaders.character.render_object(&mut render_pass, &self.character_face);
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

    let start = Instant::now();
    
    e.add_script(Scene::new(e));
    e.add_script(Camera::new(e));

    info!("Game initialized in {}ms", (Instant::now()-start).as_millis());
    
    e.start(el)
}