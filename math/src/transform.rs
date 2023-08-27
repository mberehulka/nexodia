use crate::{Vec3, Quaternion};

#[derive(Default)]
pub struct Transform {
    pub position: Vec3,
    pub scale: Vec3,
    pub rotation: Quaternion
}