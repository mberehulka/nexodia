use std::f32::consts::FRAC_PI_2;
use winit::event::VirtualKeyCode;
use engine::{Script, Engine, CameraBinding, Vec3, Mat4x4};

const CAM_MAX_ANG: f32 = FRAC_PI_2 - 0.1;

pub struct OrbitalCamera {
    e: &'static Engine,
    distance: f32,
    rotation: Vec3,
    target: Vec3
}
impl Script for OrbitalCamera {
    fn new(e: &'static Engine) -> Self {
        Self {
            e,
            distance: 4.,
            rotation: Default::default(),
            target: Default::default()
        }
    }
    fn update(&mut self) {
        let pressed = self.e.pressed.lock().unwrap();

        let s = 5. * self.e.time.delta();

        if pressed.contains(&VirtualKeyCode::A) { self.rotation.y += s }
        else if pressed.contains(&VirtualKeyCode::D) { self.rotation.y -= s }

        if pressed.contains(&VirtualKeyCode::E) {
            self.rotation.x = (self.rotation.x + s).min(CAM_MAX_ANG).max(-CAM_MAX_ANG)
        }
        else if pressed.contains(&VirtualKeyCode::Q) {
            self.rotation.x = (self.rotation.x - s).min(CAM_MAX_ANG).max(-CAM_MAX_ANG)
        }
        
        if pressed.contains(&VirtualKeyCode::W) { self.distance -= s }
        else if pressed.contains(&VirtualKeyCode::S) { self.distance += s }

        let ws = self.e.window.inner_size();
        let aspect = ws.width as f32 / ws.height as f32;
        let proj = Mat4x4::perspective(engine::deg_to_rad(90.), aspect, 0.01, 100.);
        let position = Vec3::new(0., 0., self.distance)
            .rotate_x(self.rotation.x)
            .rotate_y(self.rotation.y);
        let view = Mat4x4::look_at(self.target + position, self.target);
        self.e.update_camera_buffer(CameraBinding {
            matrix: (proj * view).into()
        })
    }
}