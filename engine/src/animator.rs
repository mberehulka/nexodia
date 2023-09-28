use math::{Mat4x4, Vec3, Quaternion};
use wgpu::{Buffer, util::DeviceExt};

use crate::{Mesh, Vertex, Engine, SkeletonJoint};

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct AnimatorFrame(pub [Mat4x4; 128]);
impl Default for AnimatorFrame {
    fn default() -> Self {
        Self([Mat4x4::IDENTITY; 128])
    }
}

pub struct Joint {
    pub parents: Vec<u8>,
    pub tpose: Mat4x4,
    pub ibm: Mat4x4,
    translation: Option<Vec3>,
    rotation: Option<Quaternion>,
    scale: Option<Vec3>,
    local_pose_cache: Option<Mat4x4>
}
impl Joint {
    pub fn new(sj: SkeletonJoint) -> Self {
        Self {
            parents: sj.parents,
            tpose: sj.tpose,
            ibm: sj.ibm,
            translation: None,
            rotation: None,
            scale: None,
            local_pose_cache: None
        }
    }
    pub fn local_pose(&self, joints: &[Joint]) -> Mat4x4 {
        match self.local_pose_cache {
            Some(v) => v,
            None => if let Some(parent) = self.parents.first() {
                self.tpose * joints[*parent as usize].ibm
            }else { self.tpose }
        }
    }
    pub fn pose(&self, joints: &[Joint]) -> Mat4x4 {
        let mut pose = Mat4x4::IDENTITY;
        for parent in self.parents.iter().rev() {
            pose = joints[*parent as usize].local_pose(joints) * pose
        }
        self.local_pose(joints) * pose
    }
}

pub struct Animator {
    pub joints: Vec<Joint>,
    pub buffer: Buffer
}
impl Animator {
    pub fn new<V: Vertex>(e: &Engine, mesh: &Mesh<V>) -> Self {
        let skeleton = mesh.skeleton.as_ref().unwrap();
        Self {
            joints: skeleton.joints.iter().map(|v|Joint::new(v.clone())).collect(),
            buffer: e.device.create_buffer_init(
                &wgpu::util::BufferInitDescriptor {
                    label: None,
                    contents: bytemuck::bytes_of(&AnimatorFrame::default()),
                    usage: wgpu::BufferUsages::UNIFORM
                }
            )
        }
    }
    pub fn get_pose(&self) -> AnimatorFrame {
        let mut frame = AnimatorFrame::default();
        for (i, joint) in self.joints.iter().enumerate() {
            frame.0[i] = joint.pose(&self.joints) * joint.ibm
        }
        frame
    }
}