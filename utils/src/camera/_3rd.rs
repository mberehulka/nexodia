use std::{f32::consts::FRAC_PI_2, sync::{Arc, Mutex}};
use winit::event::{WindowEvent, Event};
use engine::{Script, Engine, CameraBinding, Vec3, Mat4x4, Vec2, Quaternion};

use crate::Lerp;

const CAM_MAX_ANG: f32 = FRAC_PI_2 - 0.1;

#[derive(Default, Clone)]
pub struct TPCValues {
    pub target: Arc<Mutex<Vec3>>,
    pub direction: Arc<Mutex<f32>>
}

pub struct ThirdPersonCamera {
    e: &'static Engine,
    values: TPCValues,
    cursor_movement: Vec2,
    cursor_speed: Vec2,
    rotation: Lerp<Vec2>,
    distance: Lerp<f32>
}
impl ThirdPersonCamera {
    pub fn new(e: &'static Engine) -> (Self, TPCValues) {
        e.center_window();
        e.center_cursor();
        e.window.set_cursor_grab(winit::window::CursorGrabMode::Confined).ok();
        e.window.set_ime_allowed(true);
        e.window.set_cursor_visible(false);
        e.window.focus_window();
        let values = TPCValues::default();
        (
            Self {
                e,
                values: values.clone(),
                cursor_movement: Default::default(),
                cursor_speed: Vec2::new(2., 2.),
                rotation: Lerp::new(Vec2::new(0., 0.), 20.),
                distance: Lerp::new(2., 0.1)
            },
            values
        )
    }
}
impl Script for ThirdPersonCamera {
    fn event(&mut self, event: winit::event::Event<'static, ()>) {
        match event {
            Event::WindowEvent { event: WindowEvent::CursorMoved { position, .. }, .. } => {
                let ws = self.e.window.inner_size();
                self.cursor_movement = Vec2::new(
                    position.x as f32 - ws.width as f32 / 2.,
                    position.y as f32 - ws.height as f32 / 2.
                )
            }
            _ => {}
        }
    }
    fn update(&mut self) {
        self.e.center_cursor();

        let s = self.e.time.delta();

        self.rotation.target.y -= self.cursor_movement.x * self.cursor_speed.y * s;
        self.rotation.target.x = (self.rotation.target.x - self.cursor_movement.y * self.cursor_speed.x * s).min(CAM_MAX_ANG).max(-CAM_MAX_ANG);
        self.rotation.lerp(s);
        *self.values.direction.lock().unwrap() = self.rotation.y;
        let rotation = Quaternion::from_angle_y(self.rotation.y) * Quaternion::from_angle_x(-self.rotation.x);

        let mut target = {
            let t = self.values.target.lock().unwrap();
            let res = t.clone();
            drop(t);
            res
        };
        target.y += 1.;

        let position = target + rotation * Vec3::new(0., 0., *self.distance);

        let ws = self.e.window.inner_size();
        let aspect = ws.width as f32 / ws.height as f32;
        let proj = Mat4x4::perspective(aspect, aspect, 0.01, 100.);
        let view = Mat4x4::look_at(position, target);
        self.e.update_camera_buffer(CameraBinding {
            matrix: (proj * view).into(),
            position: position.extend(1.).into()
        });
        self.cursor_movement = Vec2::new(0., 0.)
    }
}