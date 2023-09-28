use std::sync::Arc;
use compiler::Joint;
use math::Mat4x4;
use wgpu::{Buffer, util::DeviceExt};

use crate::{Mesh, Vertex, Engine};

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct AnimatorFrame(pub [Mat4x4; 128]);
impl Default for AnimatorFrame {
    fn default() -> Self {
        Self([Mat4x4::IDENTITY; 128])
    }
}

#[derive(Clone)]
pub struct AnimatorJoint {
    pub parents: Vec<u8>,
    pub tpose: Mat4x4
}
impl AnimatorJoint {
    pub fn new(sj: Joint) -> Self {
        Self {
            parents: sj.parents,
            tpose: sj.pose
        }
    }
    pub fn global_pose(&self, joints: &[AnimatorJoint]) -> Mat4x4 {
        if let Some(last_parent) = self.parents.iter().last() {
            let mut pose = joints[*last_parent as usize].tpose;
            for parent in self.parents.iter().take(self.parents.len()-1).rev() {
                pose = joints[*parent as usize].tpose * pose
            }
            self.tpose * pose
        } else {
            return self.tpose
        }
    }
}

#[derive(Clone)]
pub struct Animator {
    pub joints: Vec<AnimatorJoint>,
    pub buffer: Arc<Buffer>
}
impl Animator {
    pub fn new<V: Vertex>(e: &Engine, mesh: &Mesh<V>) -> Self {
        let skeleton = mesh.skeleton.as_ref().unwrap();
        Self {
            joints: skeleton.joints.iter().map(|v|AnimatorJoint::new(v.clone())).collect(),
            buffer: e.device.create_buffer_init(
                &wgpu::util::BufferInitDescriptor {
                    label: None,
                    contents: bytemuck::bytes_of(&AnimatorFrame::default()),
                    usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST
                }
            ).into()
        }
    }
    pub fn get_frame(&self) -> AnimatorFrame {
        let mut frame = AnimatorFrame::default();
        for (i, joint) in self.joints.iter().enumerate() {
            frame.0[i] = joint.global_pose(&self.joints)
        }
        frame
    }
    pub fn update(&self, e: &Engine) {
        e.queue.write_buffer(&self.buffer, 0, bytemuck::bytes_of(&self.get_frame()))
    }
}