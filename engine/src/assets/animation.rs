use std::{path::Path, time::Instant};
use math::{Transform, Vec3, Quaternion};

use crate::{Engine, Reader};

type Frame = Vec<Transform>;

pub struct Animation {
    pub frames: Vec<Frame>
}
impl Engine {
    pub fn load_animation(&self, path: impl AsRef<Path>) -> Animation {
        let start = Instant::now();
        
        let mut r = Reader::new(path.as_ref());
        assert!(r.read_u8() == b'A');

        let joints_count = r.read_u8() as usize;
        let frames_count = r.read_u32() as usize;

        let mut frames = Vec::with_capacity(frames_count);
        for _ in 0..frames_count {
            let mut joints = Vec::with_capacity(joints_count);
            for _ in 0..joints_count {
                joints.push(Transform::new(
                    Vec3::new(r.read_f32(), r.read_f32(), r.read_f32()),
                    Quaternion::new(r.read_f32(), r.read_f32(), r.read_f32(), r.read_f32()),
                    Vec3::new(r.read_f32(), r.read_f32(), r.read_f32())
                ))
            }
            frames.push(joints)
        }

        info!("Animation '{}' loaded in: {}ms", path.as_ref().display(), (Instant::now() - start).as_millis());
        
        Animation {
            frames: frames.into()
        }
    }
}