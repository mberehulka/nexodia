use std::{f32::consts::PI, sync::Arc};
use engine::{Script, Engine, Object, utils::Color, Animator, Light, Quaternion, Vec3, ScriptHandler, Mesh};

use crate::{CameraValues, shaders::character::{Material, Shader}};

type Vertex = <Shader as engine::Shader>::Vertex;

load_animations!(
    CharacterAnimations,
    { "male/animations", idle, true }
    { "male/animations", walk_forward, true }
    { "male/animations", walk_back, true }
    { "male/animations", walk_right, true }
    { "male/animations", walk_left, true }
);

pub struct MainCharacter<'s> {
    e: &'static Engine,
    animations: Arc<CharacterAnimations>,
    camera_values: CameraValues,
    pub animator: Animator<'s>
}
impl<'s> MainCharacter<'s> {
    pub fn new(
        e: &'static Engine,
        animations: Arc<CharacterAnimations>,
        mesh: Mesh<Vertex>,
        light: &Light,
        camera_values: CameraValues
    ) -> (ScriptHandler, Object<Shader>) {
        let animator = e.animator(&mesh, animations.idle);
        let object = e.create_object(
            Material::new(e, &animator, light, Color::from("#d69f7e").into()),
            mesh
        );
        let script = e.scripts.add(MainCharacter {
            e,
            animations: animations.clone(),
            camera_values,
            animator
        });
        (script, object)
    }
}
impl<'s> Script for MainCharacter<'s> {
    fn update(&mut self) {
        if self.e.pressed_keys["W"] | self.e.pressed_keys["S"] | self.e.pressed_keys["A"] | self.e.pressed_keys["D"] {
            let target_direction = Quaternion::from_angle_y(*self.camera_values.direction.lock().unwrap() + PI).normalised();
            let t = self.e.time.delta();
            self.animator.transform.rotation = self.animator.transform.rotation.nlerp(target_direction, t * 2.).normalised();
            let s = t * 1.35;
            if self.e.pressed_keys["W"] {
                self.animator.set_animation(self.animations.walk_forward);
                self.animator.transform.translation += (self.animator.transform.rotation * Vec3::new( 0., 0.,  1.)).with_y(0.).normalized() * s
            } else if self.e.pressed_keys["S"] {
                self.animator.set_animation(self.animations.walk_back);
                self.animator.transform.translation += (self.animator.transform.rotation * Vec3::new( 0., 0., -1.)).with_y(0.).normalized() * s
            } else if self.e.pressed_keys["A"] {
                self.animator.set_animation(self.animations.walk_left);
                self.animator.transform.translation += (self.animator.transform.rotation * Vec3::new( 1., 0.,  0.)).with_y(0.).normalized() * s
            } else if self.e.pressed_keys["D"] {
                self.animator.set_animation(self.animations.walk_right);
                self.animator.transform.translation += (self.animator.transform.rotation * Vec3::new(-1., 0.,  0.)).with_y(0.).normalized() * s
            }
        } else {
            self.animator.set_animation(self.animations.idle)
        }
        self.animator.update(self.e);
        *self.camera_values.target.lock().unwrap() = self.animator.position();
    }
}