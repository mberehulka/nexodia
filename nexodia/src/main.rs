#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::time::Instant;
use engine::{Engine, Script, Instances, InstancesRenderer, Frame, Animator};
use winit::{event::VirtualKeyCode, dpi::PhysicalSize};

#[macro_use]
extern crate engine;

mod camera;   use camera::*;
mod shaders;  use shaders::*;

pub struct Scene {
    e: &'static Engine,
    frame: Frame,
    characters_shader: shaders::character::Shader,
    characters: Instances<character::Shader>,
    characters_animator: Animator
}
impl Scene {
    fn new(e: &'static Engine) -> Self {
        let tx = 5;
        let ty = 1;
        let tz = 5;
        let size = 2.;
        let space = 4.;

        let character_mesh = e.load_mesh("assets/mutant/mutant.bin");
        let characters_animator = Animator::new(e, &character_mesh);
        let character_material = character::Material::new(
            characters_animator.clone(),
            vec![ e.load_texture("assets/mutant/textures/diffuse.bin") ]
        );
        let characters_shader = character_material.create_shader(e);
        
        Self {
            e,
            frame: Frame::new(e, true),
            characters_shader,
            characters: e.create_instances(
                character_mesh,
                character_material,
                Some(
                    (0..tx).map(|x|
                        (0..ty).map(move |y|
                            (0..tz).map(move |z|
                                character::Instance {
                                    translation: [(x-tx/2) as f32*space, (y-ty/2) as f32*space, (z-tz/2) as f32*space],
                                    scale: [size;3],
                                    texture_id: 0
                                }
                            )
                        )
                    ).flatten().flatten().collect()
                )
            ),
            characters_animator
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
        self.characters_animator.update(self.e);
        self.characters.update(self.e);
        let mut render_pass = self.frame.new_render_pass(true);
        render_pass.set_bind_group(0, &self.e.camera.bind_group, &[]);
        self.characters_shader.render_instances(&mut render_pass, &self.characters);
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