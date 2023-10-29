use std::f32::consts::{FRAC_PI_2, PI};
use winit::event::{WindowEvent, Event};
use engine::{Script, Engine, CameraBinding, Vec3, Mat4x4, Vec2};

const CAM_MAX_ANG: f32 = FRAC_PI_2 - 0.1;

pub struct FirstPersonCamera {
    e: &'static Engine,

    rotation: Vec3,
    target_rotation: Vec3,
    rotation_smoothness: f32,

    translation: Vec3,
    translation_speed: Vec3,
    target_translation: Vec3,
    translation_smoothness: f32,
    
    target: Vec3,
    cursor_movement: Vec2,
    cursor_speed: Vec2
}
impl FirstPersonCamera {
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
            target_rotation: Vec3::new(0., PI, 0.),
            rotation_smoothness: 20.,
            
            translation: Vec3::new(0., 3., 2.),
            translation_speed: [3.;3].into(),
            target_translation: Vec3::new(0., 3., 2.),
            translation_smoothness: 20.,
            
            target: Default::default(),
            cursor_movement: Default::default(),
            cursor_speed: Vec2::new(2., 2.)
        }
    }
}
impl Script for FirstPersonCamera {
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

        self.target_rotation.y += self.cursor_movement.x * self.cursor_speed.y * s;
        self.target_rotation.x = (self.target_rotation.x - self.cursor_movement.y * self.cursor_speed.x * s).min(CAM_MAX_ANG).max(-CAM_MAX_ANG);

        self.rotation.lerp(self.target_rotation, self.rotation_smoothness * s);
        
        if self.e.pressed_keys["E"] { self.target_translation.y += self.translation_speed.y * s }
        if self.e.pressed_keys["Q"] { self.target_translation.y -= self.translation_speed.y * s }
        if self.e.pressed_keys["W"] {
            self.target_translation += Vec3::new(0., 0., self.translation_speed.z).rotate_y(self.rotation.y) * s
        }
        if self.e.pressed_keys["S"] {
            self.target_translation -= Vec3::new(0., 0., self.translation_speed.z).rotate_y(self.rotation.y) * s
        }
        if self.e.pressed_keys["A"] {
            self.target_translation += Vec3::new(self.translation_speed.x, 0., 0.).rotate_y(self.rotation.y) * s
        }
        if self.e.pressed_keys["D"] {
            self.target_translation -= Vec3::new(self.translation_speed.x, 0., 0.).rotate_y(self.rotation.y) * s
        }

        self.translation.lerp(self.target_translation, self.translation_smoothness * s);
        
        let ws = self.e.window.inner_size();
        let aspect = ws.width as f32 / ws.height as f32;
        let proj = Mat4x4::perspective(aspect, aspect, 0.01, 100.);
        let position = self.target + self.translation;
        let view = Mat4x4::look_at(position, position + Vec3::new(0., 0., 1.).rotate_x(self.rotation.x).rotate_y(self.rotation.y));
        self.e.update_camera_buffer(CameraBinding {
            matrix: (proj * view).into(),
            position: position.extend(1.).into()
        });
        
        self.cursor_movement = Vec2::new(0., 0.)
    }
}