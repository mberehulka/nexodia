use std::{path::Path, sync::Arc};

use compiler::Skeleton;
use math::Transform;
use wgpu::{Buffer, util::DeviceExt};

use crate::{Mesh, Vertex, Engine, Animation};

pub const MAX_JOINTS: usize = 96;  // 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 48, 64, 96, 128, 256, 512, 1024, 2048, 4096

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
    pub animations: Vec<Animation>,
    pub skeleton: Skeleton,
    pub transform: Transform,
    pub buffer: Arc<Buffer>,
    time: f32,
    pub speed: f32,
    current_animation: usize
}
impl Animator {
    pub fn new<V: Vertex>(e: &Engine, mesh: &Mesh<V>, animations: Vec<Animation>) -> Self {
        Self {
            animations,
            skeleton: mesh.skeleton.clone().unwrap(),
            transform: Default::default(),
            buffer: e.device.create_buffer_init(
                &wgpu::util::BufferInitDescriptor {
                    label: None,
                    contents: bytemuck::bytes_of(&AnimatorBindingFrame::default()),
                    usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST
                }
            ).into(),
            time: 0.,
            speed: 30.,
            current_animation: 0
        }
    }
    pub fn set_animation(&mut self, animation: usize) {
        if animation != self.current_animation {
            self.current_animation = animation;
            self.time  = 0.
        }
    }
    fn get_binding_frame(&mut self, e: &Engine) -> AnimatorBindingFrame {
        let cur_animation = &self.animations[self.current_animation];

        // Update time
        self.time += e.time.delta() * self.speed;
        if self.time as usize >= cur_animation.frames.len() - 1 {
            self.time = 0.
        }
        let time_frac = self.time - self.time.floor();
        let time = self.time as usize;

        // Get lerped frames
        let frame = cur_animation.lerp_frames(time, time + 1, time_frac);
        let transform = self.transform * cur_animation.lerp_root(time, time + 1, time_frac);
        
        // Calculate global pose
        let mut binding_frame = AnimatorBindingFrame::default();
        for i in 0..frame.len() {
            binding_frame.joints[i] = (transform * (frame[i].model * self.skeleton.joints[i].ibm)).into()
        }
        binding_frame
    }
    pub fn update(&mut self, e: &Engine) {
        let binding_frame = self.get_binding_frame(e);
        e.queue.write_buffer(&self.buffer, 0, bytemuck::bytes_of(&binding_frame))
    }
}

impl Engine {
    pub fn load_animations<V: Vertex>(&self, mesh: &Mesh<V>, path: impl AsRef<Path>) -> Animator {
        let animations = std::fs::read_dir(path.as_ref()).unwrap()
            .into_iter()
            .map(|path| path.unwrap().path())
            .filter(|path| path.extension().unwrap().to_str().unwrap() == "bin")
            .map(|path| self.load_animation(path))
            .collect();
        Animator::new(self, mesh, animations)
    }
}