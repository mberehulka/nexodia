use std::{path::Path, sync::Arc};

use compiler::Joint;
use math::{Mat4x4, Transform};
use wgpu::{Buffer, util::DeviceExt};

use crate::{Mesh, Vertex, Engine, Animation};

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct AnimatorBindingFrame(pub [[[f32;4];4]; 128]);
impl Default for AnimatorBindingFrame {
    fn default() -> Self {
        Self([Mat4x4::IDENTITY.into(); 128])
    }
}

pub struct AnimatorJoint {
    pub id: usize,
    pub parents: Vec<u8>,    // from nearest to further
    pub local_pose: Transform,
    pub global_ibm: Mat4x4
}
impl AnimatorJoint {
    pub fn new(sj: Joint, pose: Transform, id: usize) -> Self {
        Self {
            id,
            parents: sj.parents,
            local_pose: pose,
            global_ibm: sj.global_ibm
        }
    }
    pub fn global_pose(&self, joints: &[AnimatorJoint]) -> Transform {
        self.parents.iter()
            .map(|i| joints[*i as usize].local_pose)
            .fold(self.local_pose, |acc, e| acc * e)
    }
}

pub struct Animator {
    pub animations: Vec<Animation>,
    pub transform: Transform,
    pub joints: Vec<AnimatorJoint>,
    pub buffer: Arc<Buffer>,
    time: f32,
    pub speed: f32,
    current_animation: usize
}
impl Animator {
    pub fn new<V: Vertex>(e: &Engine, mesh: &Mesh<V>, animations: Vec<Animation>) -> Self {
        let skeleton = mesh.skeleton.as_ref().unwrap();
        let first_frame = animations.first().unwrap().frames.first().unwrap();
        Self {
            transform: Default::default(),
            joints: skeleton.joints.iter()
                .zip(first_frame.iter())
                .enumerate()
                .map(|(i, (joint, pose))|AnimatorJoint::new(joint.clone(), *pose, i))
                .collect(),
            animations: animations.into(),
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

        // Update joints local pose
        self.time += e.time.delta() * self.speed;
        if self.time as usize >= cur_animation.frames.len() - 1 {
            self.time = 0.
        }
        let time_frac = self.time - self.time.floor();

        for ((joint, pose), next_pose) in
            self.joints.iter_mut()
                .zip(cur_animation.frames[self.time as usize].iter())
                .zip(cur_animation.frames[self.time as usize + 1].iter())
        {
            joint.local_pose = pose.lerp(*next_pose, time_frac)
        }

        // Calculate global pose
        let mut binding_frame = AnimatorBindingFrame::default();
        for (i, joint) in self.joints.iter().enumerate() {
            binding_frame.0[i] = (self.transform * (joint.global_pose(&self.joints) * joint.global_ibm)).into()
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