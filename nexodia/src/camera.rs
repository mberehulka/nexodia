use std::f32::consts::{FRAC_PI_2, PI};
use winit::event::{WindowEvent, Event};
use engine::{Script, Engine, CameraBinding, Vec3, Mat4x4, Vec2};

const CAM_MAX_ANG: f32 = FRAC_PI_2 - 0.1;

pub struct Camera {
    e: &'static Engine,
    rotation: Vec3,
    translation: Vec3,
    target: Vec3,
    cursor_movement: Vec2,
    cursor_speed: Vec2
}
impl Camera {
    pub fn new(e: &'static Engine) -> Self {
        e.center_window();
        e.center_cursor();
        e.window.set_cursor_grab(winit::window::CursorGrabMode::Confined).ok();
        e.window.set_ime_allowed(true);
        e.window.focus_window();
        e.window.set_cursor_visible(false);
        Self {
            e,
            rotation: Vec3::new(0., PI, 0.),
            translation: Vec3::new(0., 3., 2.),
            target: Default::default(),
            cursor_movement: Default::default(),
            cursor_speed: Vec2::new(0.1, 0.1)
        }
    }
}
impl Script for Camera {
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

        let s = 3. * self.e.time.delta();

        self.rotation.y += self.cursor_movement.x * self.cursor_speed.y * s;
        self.rotation.x = (self.rotation.x - self.cursor_speed.x * self.cursor_movement.y * s).min(CAM_MAX_ANG).max(-CAM_MAX_ANG);

        if self.e.pressed_keys["E"] { self.translation.y += s }
        else if self.e.pressed_keys["Q"] { self.translation.y -= s }
        
        if self.e.pressed_keys["W"] {
            self.translation += Vec3::new(0., 0., 1.).rotate_y(self.rotation.y) * s
        } else if self.e.pressed_keys["S"] {
            self.translation -= Vec3::new(0., 0., 1.).rotate_y(self.rotation.y) * s
        }
        if self.e.pressed_keys["A"] {
            self.translation += Vec3::new(1., 0., 0.).rotate_y(self.rotation.y) * s
        } else if self.e.pressed_keys["D"] {
            self.translation -= Vec3::new(1., 0., 0.).rotate_y(self.rotation.y) * s
        }

        let ws = self.e.window.inner_size();
        let aspect = ws.width as f32 / ws.height as f32;
        let proj = Mat4x4::perspective(aspect, aspect, 0.01, 100.);
        let position = self.target + self.translation;
        let view = Mat4x4::look_at(position, position + Vec3::new(0., 0., 1.).rotate_x(self.rotation.x).rotate_y(self.rotation.y));
        self.e.update_camera_buffer(CameraBinding {
            matrix: (proj * view).into()
        });
        
        self.cursor_movement = Vec2::new(0., 0.)
    }
}