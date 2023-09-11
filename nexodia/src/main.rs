#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::time::Instant;
use engine::{Engine, Script, Instances, InstancesRenderer, Frame};
use winit::{event::VirtualKeyCode, dpi::PhysicalSize};

#[macro_use]
extern crate engine;

mod camera;   use camera::*;
mod shaders;  use shaders::*;

pub struct Scene {
    e: &'static Engine,
    frame: Frame,
    shaders: Shaders,
    characters: Instances<instance_t_f_pu::Shader>
}
impl Scene {
    fn new(e: &'static Engine) -> Self {
        let tx = 5;
        let ty = 1;
        let tz = 5;
        let size = 2.;
        let space = 4.;
        Self {
            e,
            frame: Frame::new(e, true),
            shaders: Shaders::new(e),
            characters: e.create_instances(
                e.load_mesh("assets/mannequin/mannequin.bin"),
                instance_t_f_pu::Material(vec![
                    e.load_texture("assets/mannequin/textures/Ch36_1001_Diffuse.bin")
                ]),
                Some(
                    (0..tx).map(|x|
                        (0..ty).map(move |y|
                            (0..tz).map(move |z|
                                instance_t_f_pu::Instance {
                                    translation: [(x-tx/2) as f32*space, (y-ty/2) as f32*space, (z-tz/2) as f32*space],
                                    scale: [size;3],
                                    texture_id: 0
                                }
                            )
                        )
                    ).flatten().flatten().collect()
                )
            )
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
        self.characters.update(self.e);
        let mut render_pass = self.frame.new_render_pass(true);
        render_pass.set_bind_group(0, &self.e.camera.bind_group, &[]);
        self.shaders.instance_t_f_pu.render_instances(&mut render_pass, &self.characters);
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