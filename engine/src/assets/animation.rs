use std::path::Path;
use math::Transform;

use crate::{Engine, Reader};

#[derive(Default, Clone)]
pub struct AnimationJoint {
    pub model_space: Transform   // model space joint matrix
}

#[derive(Default, Clone)]
// Vector of Joints Transformation
pub struct AnimationFrame {
    pub root: AnimationJoint,
    pub joints: Vec<AnimationJoint>
}
impl AnimationFrame {
    pub fn lerp(&mut self, next: &Self, amount: f32) {
        let joints_length = self.joints.len();
        for i in 0..joints_length {
            self.joints[i].model_space.lerp(next.joints[i].model_space, amount)
        }
        self.root.model_space.lerp(next.root.model_space, amount)
    }
}

pub struct Animation {
    pub frames: Vec<AnimationFrame>
}
impl Engine {
    pub fn load_animation(&self, path: impl AsRef<Path>) -> Animation {
        let mut r = Reader::new(path.as_ref());
        assert!(r.read_u8() == b'A');

        let joints = r.read_u8() as usize;
        let frames = r.read_u32() as usize;

        Animation {
            frames: (0..frames).map(|_| {
                AnimationFrame {
                    root: r.read_animation_joint(),
                    joints: (0..joints).map(|_| r.read_animation_joint() ).collect()
                }
            }).collect()
        }
    }
}