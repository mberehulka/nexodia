use std::sync::Arc;
use compiler::Skeleton;
use math::{SimpleTransform, Vec3};
use wgpu::{Buffer, util::DeviceExt};
use crate::{Mesh, Vertex, Engine, Animation, AnimationFrame};

pub const MAX_JOINTS: usize = 96;  // 16, 17, ..., 31, 32, 48, 64, 96, 128, 256, 512, 1024, 2048, 4096

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct AnimatorBindingFrame {
    pub joints: [[[f32;4];4]; MAX_JOINTS]
}
impl Default for AnimatorBindingFrame {
    fn default() -> Self {
        Self {
            joints: [Default::default(); MAX_JOINTS]
        }
    }
}

pub struct Animator {
    pub skeleton: Arc<Skeleton>,
    pub transform: SimpleTransform,
    pub buffer: Arc<Buffer>,
    pub speed: f32,
    pub frame: AnimationFrame,
    pub animation: Animation,
    pub time: f32
}
impl Animator {
    pub fn new<V: Vertex>(e: &Engine, mesh: &Mesh<V>, animation: Animation) -> Self {
        Self {
            skeleton: mesh.skeleton.as_ref().unwrap().clone(),
            transform: Default::default(),
            buffer: e.device.create_buffer_init(
                &wgpu::util::BufferInitDescriptor {
                    label: None,
                    contents: bytemuck::bytes_of(&AnimatorBindingFrame::default()),
                    usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST
                }
            ).into(),
            speed: 30.,
            frame: animation.frames.first().unwrap().clone(),
            animation,
            time: 0.
        }
    }
    fn finished(&self) -> bool {
        self.time as usize >= self.animation.frames.len()
    }
    fn is_last_frame(&self) -> bool {
        self.time as usize >= self.animation.frames.len().saturating_sub(1)
    }
    pub fn current_frame(&self) -> &AnimationFrame {
        &self.animation.frames[self.time as usize]
    }
    fn next_frame(&self) -> &AnimationFrame {
        &self.animation.frames[if self.is_last_frame() { 0 } else { self.time as usize + 1 }]
    }
    pub fn position(&self) -> Vec3 {
        self.transform * self.frame.root.translation
    }
    pub fn transform(&self) -> SimpleTransform {
        self.transform * self.frame.root
    }
    fn reset_animation(&mut self) {
        self.time = 0.
    }
    pub fn set_animation(&mut self, animation: Animation) {
        if animation.id != self.animation.id {
            self.reset_animation();
            self.animation = animation
        }
    }
    fn get_binding_frame(&mut self, delta_time: f32) -> AnimatorBindingFrame {
        let mut cur_frame = self.current_frame().clone();
        let next_frame = self.next_frame();
        
        cur_frame.lerp(&next_frame, self.time.fract());
        self.frame.lerp(&cur_frame, delta_time * 10.);

        let transform = self.transform * self.frame.root;
        
        let mut binding_frame = AnimatorBindingFrame::default();
        for i in 0..cur_frame.joints.len() {
            binding_frame.joints[i] = (transform * (self.frame.joints[i] * self.skeleton.joints[i].ibm)).into()
        }
        binding_frame
    }
    fn update_time(&mut self, delta_time: f32) {
        self.time += delta_time * self.speed;
        if self.finished() {
            self.reset_animation()
        }
    }
    pub fn update(&mut self, e: &Engine) {
        let delta_time = e.time.delta();
        let binding_frame = self.get_binding_frame(delta_time);
        self.update_time(delta_time);
        e.queue.write_buffer(&self.buffer, 0, bytemuck::bytes_of(&binding_frame))
    }
}

impl Engine {
    pub fn animator<V: Vertex>(&self, mesh: &Mesh<V>, animation: Animation) -> Animator {
        Animator::new(self, mesh, animation)
    }
}