use std::sync::Arc;
use winit::event::VirtualKeyCode;
use engine::{Engine, Script, Quaternion, Light, ObjectRenderer, ScriptInstance, utils::{Id, new_render_pass}};

use crate::{
    objects::{BasicObject, Character, ThirdPersonCamera, MainCharacter, CameraValues},
    shaders::Shaders
};

assets!(
    animations [
        male/animations/idle,
        male/animations/walk_forward,
        male/animations/walk_back,
        male/animations/walk_right,
        male/animations/walk_left
    ]
    meshes [
        male/base/base  > engine::vertex::pnj::Vertex,
        geometries/cube > engine::vertex::pu::Vertex
    ]
    textures [
        textures/grass
    ]
);

pub struct Scene {
    e: &'static Engine,
    _camera: ScriptInstance<CameraValues>,
    shaders: Shaders,
    scenary: Vec<BasicObject>,
    dir_light: Light,
    main_char: ScriptInstance<Character>
}
impl<'s> Script<'s> for Scene {
    type Params = ();
    type Return = ();
    const NAME: &'static str = "MainScene";
    fn new(e: &'static Engine, _id: Id, _params: Self::Params) -> (Self, Self::Return) {
        let camera = e.new_script::<ThirdPersonCamera>(());
        let dir_light = Light::new(e, Quaternion::from_angle_x(0.), (1024, 1024));

        let assets = Arc::new(Assets::new(e));
        
        let main_char = e.new_script::<MainCharacter>((
            assets.clone(),
            assets.male_base_base.clone(),
            &dir_light,
            camera.0.clone()
        ));
        
        (
            Self {
                e,
                _camera: camera,
                shaders: Shaders::new(e),
                scenary: vec![
                    BasicObject::new(e, &dir_light, assets.textures_grass.clone(), assets.geometries_cube.clone())
                ],
                dir_light,
                main_char
            },
            ()
        )
    }
    fn render(&mut self) {
        if self.e.pressed_keys[VirtualKeyCode::Escape] { self.e.exit() }
        self.e.render(move |encoder| {
            {
                let mut render_pass = new_render_pass(
                    encoder,
                    None,
                    Some(&self.dir_light.depth_texture.view)
                );
                self.shaders.character.dir_light.render_object(&mut render_pass, &self.main_char.0.dir_light);
                for object in self.scenary.iter() {
                    self.shaders.basic.dir_light.render_object(&mut render_pass, &object.dir_light)
                }
            }
            let depth_texture = self.e.depth_texture.lock().unwrap();
            let output_texture = self.e.output_texture.lock().unwrap();
            let mut render_pass = new_render_pass(
                encoder,
                Some(&output_texture.view),
                Some(&depth_texture.view)
            );
            render_pass.set_bind_group(0, &self.e.camera_buffer.bind_group, &[]);
            self.shaders.character.main.render_object(&mut render_pass, &self.main_char.0.main);
            for object in self.scenary.iter() {
                self.shaders.basic.main.render_object(&mut render_pass, &object.main)
            }
        });
    }
}