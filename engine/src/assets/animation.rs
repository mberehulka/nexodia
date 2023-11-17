use std::{path::Path, sync::Arc};
use math::{SimpleTransform, Vec3};

use crate::{Engine, Reader, utils::{IdHandler, Id}};

static ID: IdHandler = IdHandler::default();

#[derive(Default, Clone)]
pub struct AnimationFrame {
    pub root: SimpleTransform,
    pub joints: Vec<SimpleTransform>
}
impl AnimationFrame {
    pub fn lerp(&mut self, next: &Self, amount: f32) {
        for i in 0..self.joints.len() {
            self.joints[i].lerp(next.joints[i], amount)
        }
        self.root.lerp(next.root, amount)
    }
    pub fn lerp_joints(&mut self, next: &Self, amount: f32) {
        for i in 0..self.joints.len() {
            self.joints[i].lerp(next.joints[i], amount)
        }
    }
}

#[derive(Clone)]
pub struct Animation {
    pub id: Id,
    pub frames: Arc<Vec<AnimationFrame>>,
    pub keep_root_translation_axis: Vec3
}
impl Engine {
    pub fn load_animation(&self, path: impl AsRef<Path>) -> Animation {
        let mut r = Reader::new(path.as_ref());
        assert!(r.read_u8() == b'A');

        let joints = r.read_u8() as usize;
        let frames = r.read_u32() as usize;

        Animation {
            id: ID.next(),
            frames: (0..frames).map(|_| {
                AnimationFrame {
                    root: r.read_transform(),
                    joints: (0..joints).map(|_| r.read_transform() ).collect()
                }
            }).collect::<Vec<_>>().into(),
            keep_root_translation_axis: Default::default()
        }
    }
}