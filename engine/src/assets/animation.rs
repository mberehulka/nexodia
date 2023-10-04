use std::{path::Path, time::Instant};
use math::Transform;

use crate::{Engine, Reader};

pub struct AnimationJoint {
    pub local: Transform,  // local space joint matrix
    pub model: Transform   // model space joint matrix
}

// Vector of Joints Transformation
type Frame = Vec<AnimationJoint>;

pub struct Animation {
    pub root: Vec<Transform>,
    pub frames: Vec<Frame>
}
impl Engine {
    pub fn load_animation(&self, path: impl AsRef<Path>) -> Animation {
        let start = Instant::now();
        
        let mut r = Reader::new(path.as_ref());
        assert!(r.read_u8() == b'A');

        let joints_count = r.read_u8() as usize;
        let frames_count = r.read_u32() as usize;

        let mut root = Vec::with_capacity(frames_count);
        let mut frames = Vec::with_capacity(frames_count);
        for _ in 0..frames_count {
            root.push(r.read_transform());

            let mut joints = Vec::with_capacity(joints_count);
            for _ in 0..joints_count {
                joints.push(AnimationJoint {
                    local: r.read_transform(),
                    model: r.read_transform()
                })
            }
            frames.push(joints)
        }

        info!("Animation '{}' loaded in: {}ms", path.as_ref().display(), (Instant::now() - start).as_millis());
        
        Animation {
            frames: frames.into(),
            root
        }
    }
}
impl Animation {
    pub fn lerp_root(&self, from: usize, to: usize, amount: f32) -> Transform {
        self.root[from].lerp(self.root[to], amount)
    }
    pub fn lerp_frames(&self, from: usize, to: usize, amount: f32) -> Frame {
        let mut frame = Frame::with_capacity(self.frames[0].len());
        for (cur_pose, next_pose) in self.frames[from].iter().zip(self.frames[to].iter())
        {
            frame.push(AnimationJoint {
                local: Transform::default(),
                model: cur_pose.model.lerp(next_pose.model, amount)
            })
        }
        frame
    }
}