#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use winit::event::VirtualKeyCode;
#[macro_use]
extern crate engine;
use engine::{Engine, Script, Quaternion, Light, Transform, ObjectRenderer};

#[macro_use]
mod utils;
mod shaders;  use shaders::*;
mod objects;  use objects::*;

pub struct Scene {
    e: &'static Engine,
    _camera: Camera,
    shaders: Shaders,
    scenary: Vec<BasicObject>,
    _light: Light,
    main_char: Character
}
impl Scene {
    fn new(e: &'static Engine) -> Self {
        let camera = ThirdPersonCamera::new(e);

        let dir_light = Light::new(e, Quaternion::from_angle_x(0.), (1024, 1024));
        
        let char_animations = CharacterAnimations::new(e).into();
        let char_mesh = e.load_mesh("assets/male/base/base.bin");
        let main_char = MainCharacter::new(e, char_animations, char_mesh, &dir_light, camera.1.clone());
        
        Self {
            e,
            _camera: camera,
            shaders: Shaders::new(e),
            scenary: vec![
                BasicObject::new(e, &dir_light,
                    e.load_texture("assets/textures/grass.bin"),
                    e.transformed_mesh("assets/geometries/cube.bin", Transform::from_scale(5., 0., 5.))
                )
            ],
            _light: dir_light,
            main_char
        }
    }
}
impl Script for Scene {
    fn name() -> &'static str { "MainScene" }
    fn on_key_press(&mut self, key: VirtualKeyCode) {
        if let VirtualKeyCode::Escape = key { self.e.exit() }
    }
    fn render(&mut self) {
        self.e.render(move |encoder| {
            {
                let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: None,
                    color_attachments: &[],
                    depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                        view: &self._light.depth_texture.view,
                        depth_ops: Some(wgpu::Operations {
                            load: wgpu::LoadOp::Clear(1.),
                            store: true
                        }),
                        stencil_ops: None
                    })
                });
                self.shaders.character.dir_light.render_object(&mut render_pass, &self.main_char.dir_light);
                for object in self.scenary.iter() {
                    self.shaders.basic.dir_light.render_object(&mut render_pass, &object.dir_light)
                }
            }
            let depth_texture = self.e.depth_texture.lock().unwrap();
            let output_texture = self.e.output_texture.lock().unwrap();
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &output_texture.view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: true
                    }
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &depth_texture.view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.),
                        store: true
                    }),
                    stencil_ops: None
                })
            });
            render_pass.set_bind_group(0, &self.e.camera_buffer.bind_group, &[]);
            self.shaders.character.main.render_object(&mut render_pass, &self.main_char.main);
            for object in self.scenary.iter() {
                self.shaders.basic.main.render_object(&mut render_pass, &object.main)
            }
        })
    }
}

fn main() {
    let (el, e) = engine::Engine::new();
    e.scripts.add(Scene::new(e)).make_static();
    e.start(el)
}