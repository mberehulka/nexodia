#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{time::Instant, f32::consts::PI};
use winit::{event::VirtualKeyCode, dpi::PhysicalSize};
use engine::{
    Engine, Script, Frame, Object, ObjectRenderer, Quaternion, Animator,
    Light, utils::Color, decode, Transform, Vec3, ScriptHandler
};
use utils::{ThirdPersonCamera, TPCValues};

#[macro_use]
extern crate engine;

mod shaders;  use shaders::*;

load_animations!(
    { "male/animations", idle, true }
    { "male/animations", walk_forward, true }
    { "male/animations", walk_back, true }
    { "male/animations", walk_right, true }
    { "male/animations", walk_left, true }
);

pub struct Scene<'s> {
    e: &'static Engine,
    camera: (ScriptHandler, TPCValues),
    animations: Animations,
    frame: Frame,
    shaders: Shaders,
    character: Object<character::Shader>,
    character_animator: Animator<'s>,
    scenary: Vec<Object<basic::Shader>>
}
impl<'s> Scene<'s> {
    fn new(e: &'static Engine, animations: Animations) -> Self {
        let (camera, camera_values) = ThirdPersonCamera::new(e);
        let camera = e.scripts.add(camera);

        let character_mesh = e.load_mesh("assets/male/base/base.bin");
        let character_animator = e.animator(&character_mesh, animations.idle);

        let light = Light::new(e, Quaternion::from_angle_x(0.));

        let character: Object<character::Shader> = e.create_object(
            character::Material::new(e, &character_animator, &light),
            character_mesh
        );
        character.material.update(e, character::MaterialBinding {
            color: Color::from("#d69f7e").into()
        });
        
        Self {
            e,
            camera: (camera, camera_values),
            animations,
            frame: Frame::new(e, true),
            shaders: Shaders::new(e),
            character,
            character_animator,
            scenary: vec![
                e.create_object(
                    basic::Material::new(e, e.load_texture("assets/textures/grass.bin")),
                    e.initialize_mesh(
                        decode::<engine::compiler::Mesh>("assets/geometries/cube.bin")
                            .transform(Transform {
                                scale: Vec3::new(5., 0., 5.),
                                ..Default::default()
                            })
                    )
                )
            ]
        }
    }
}
impl<'s> Script for Scene<'s> {
    fn on_key_press(&mut self, key: VirtualKeyCode) {
        if let VirtualKeyCode::Escape = key { self.e.exit() }
    }
    fn window_resized(&mut self, _new_size: PhysicalSize<u32>) {
        self.frame.window_resized()
    }
    fn update(&mut self) {
        if self.e.pressed_keys["W"] | self.e.pressed_keys["S"] | self.e.pressed_keys["A"] | self.e.pressed_keys["D"] {
            let target_direction = Quaternion::from_angle_y(*self.camera.1.direction.lock().unwrap() + PI).normalised();
            let t = self.e.time.delta();
            self.character_animator.transform.rotation = self.character_animator.transform.rotation.nlerp(target_direction, t * 2.).normalised();
            let s = t * 1.35;
            if self.e.pressed_keys["W"] {
                self.character_animator.set_animation(self.animations.walk_forward);
                self.character_animator.transform.translation += (
                    self.character_animator.transform.rotation *
                    Vec3::new(0., 0., 1.)
                ).with_y(0.).normalized() * s
            } else if self.e.pressed_keys["S"] {
                self.character_animator.set_animation(self.animations.walk_back);
                self.character_animator.transform.translation += (
                    self.character_animator.transform.rotation *
                    Vec3::new(0., 0., -1.)
                ).with_y(0.).normalized() * s
            } else if self.e.pressed_keys["A"] {
                self.character_animator.set_animation(self.animations.walk_left);
                self.character_animator.transform.translation += (
                    self.character_animator.transform.rotation *
                    Vec3::new(1., 0., 0.)
                ).with_y(0.).normalized() * s
            } else if self.e.pressed_keys["D"] {
                self.character_animator.set_animation(self.animations.walk_right);
                self.character_animator.transform.translation += (
                    self.character_animator.transform.rotation *
                    Vec3::new(-1., 0., 0.)
                ).with_y(0.).normalized() * s
            }
        } else {
            self.character_animator.set_animation(self.animations.idle)
        }
        self.character_animator.update(self.e);

        *self.camera.1.target.lock().unwrap() = self.character_animator.position();
        
        let mut render_pass = self.frame.new_render_pass(true);
        render_pass.set_bind_group(0, &self.e.camera.bind_group, &[]);
        self.shaders.character.render_object(&mut render_pass, &self.character);
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
    
    let animations = Animations::new(e);
    
    let _main_scene = e.scripts.add(Scene::new(e, animations));

    info!("Game initialized in {}ms", (Instant::now()-start).as_millis());
    
    e.start(el)
}