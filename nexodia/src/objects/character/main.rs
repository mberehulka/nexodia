use std::{f32::consts::PI, sync::Arc};
use engine::{Script, Engine, utils::Id, Animator, Light, Quaternion, Vec3, Mesh};

use crate::{objects::CameraValues, shaders::character, objects::Character, scenes::main::Assets};

pub struct MainCharacter {
    e: &'static Engine,
    assets: Arc<Assets>,
    camera_values: CameraValues,
    pub animator: Animator
}
impl<'s> Script<'s> for MainCharacter {
    type Params = (
        Arc<Assets>,
        Mesh<<character::main::Shader as engine::Shader>::Vertex>,
        &'s Light,
        CameraValues
    );
    type Return = Character;
    const NAME: &'static str = "MainCharacter";
    fn new(
        e: &'static Engine,
        _id: Id,
        (assets, mesh, light, camera_values): Self::Params
    ) -> (Self, Self::Return) {
        let animator = e.animator(&mesh, assets.male_animations_idle.clone());
        let object = e.create_object(
            character::main::Material::new(e, &animator, light, "#d69f7e"),
            mesh.clone()
        );
        let object_light = e.create_object(
            character::dir_light::Material::new(e, &animator, light),
            mesh
        );
        (
            Self {
                e,
                assets,
                camera_values,
                animator
            },
            Character {
                main: object,
                dir_light: object_light
            }
        )
    }
    fn update(&mut self) {
        if self.e.pressed_keys["W"] | self.e.pressed_keys["S"] | self.e.pressed_keys["A"] | self.e.pressed_keys["D"] {
            let target_direction = Quaternion::from_angle_y(*self.camera_values.direction.lock().unwrap() + PI).normalised();
            let t = self.e.time.delta();
            self.animator.transform.rotation = self.animator.transform.rotation.nlerp(target_direction, t * 2.).normalised();
            let s = t * 1.35;
            if self.e.pressed_keys["W"] {
                self.animator.set_animation(self.assets.male_animations_walk_forward.clone());
                self.animator.transform.translation += (self.animator.transform.rotation * Vec3::new( 0., 0.,  1.)).with_y(0.).normalized() * s
            } else if self.e.pressed_keys["S"] {
                self.animator.set_animation(self.assets.male_animations_walk_back.clone());
                self.animator.transform.translation += (self.animator.transform.rotation * Vec3::new( 0., 0., -1.)).with_y(0.).normalized() * s
            } else if self.e.pressed_keys["A"] {
                self.animator.set_animation(self.assets.male_animations_walk_left.clone());
                self.animator.transform.translation += (self.animator.transform.rotation * Vec3::new( 1., 0.,  0.)).with_y(0.).normalized() * s
            } else if self.e.pressed_keys["D"] {
                self.animator.set_animation(self.assets.male_animations_walk_right.clone());
                self.animator.transform.translation += (self.animator.transform.rotation * Vec3::new(-1., 0.,  0.)).with_y(0.).normalized() * s
            }
        } else {
            self.animator.set_animation(self.assets.male_animations_idle.clone())
        }
        self.animator.update(self.e);
        *self.camera_values.target.lock().unwrap() = self.animator.position();
    }
}