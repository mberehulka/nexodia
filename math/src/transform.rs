use std::ops::Mul;
use bincode::{Encode, Decode};

use crate::{Vec3, Quaternion, Mat4x4, Vec4};

#[derive(Default, Copy, Clone, Encode, Decode)]
pub struct Transform {
    pub translation: Vec3,
    pub rotation: Quaternion,
    pub scale: Vec3
}
impl Transform {
    #[inline(always)]
    pub const fn new(translation: Vec3, rotation: Quaternion, scale: Vec3) -> Self {
        Self { translation, rotation, scale }
    }
    #[inline(always)]
    pub fn lerp(self, other: Self, amount: f32) -> Self {
        Self {
            translation: self.translation.lerp(other.translation, amount),
            rotation: self.rotation.lerp(other.rotation, amount),
            scale: self.scale.lerp(other.scale, amount)
        }
    }
    #[inline(always)]
    pub fn apply_translation_rotation(self, other: Vec3) -> Vec3 {
        self.translation + (self.rotation * other)
    }
}
impl From<Transform> for Mat4x4 {
    fn from(t: Transform) -> Mat4x4 {
        let x2 = t.rotation.v.x + t.rotation.v.x;
        let y2 = t.rotation.v.y + t.rotation.v.y;
        let z2 = t.rotation.v.z + t.rotation.v.z;
        let xx2 = x2 * t.rotation.v.x;
        let xy2 = x2 * t.rotation.v.y;
        let xz2 = x2 * t.rotation.v.z;
        let yy2 = y2 * t.rotation.v.y;
        let yz2 = y2 * t.rotation.v.z;
        let zz2 = z2 * t.rotation.v.z;
        let sy2 = y2 * t.rotation.s;
        let sz2 = z2 * t.rotation.s;
        let sx2 = x2 * t.rotation.s;
        Mat4x4::new(
            Vec4::new((1. - yy2 - zz2) * t.scale.x, (     xy2 + sz2) * t.scale.x, (     xz2 - sy2) * t.scale.x, 0.),
            Vec4::new((     xy2 - sz2) * t.scale.y, (1. - xx2 - zz2) * t.scale.y, (     yz2 + sx2) * t.scale.y, 0.),
            Vec4::new((     xz2 + sy2) * t.scale.z, (     yz2 - sx2) * t.scale.z, (1. - xx2 - yy2) * t.scale.z, 0.),
            t.translation.extend(1.)
        )
    }
}
impl From<Transform> for [[f32;4];4] {
    fn from(t: Transform) -> [[f32;4];4] {
        let x2 = t.rotation.v.x + t.rotation.v.x;
        let y2 = t.rotation.v.y + t.rotation.v.y;
        let z2 = t.rotation.v.z + t.rotation.v.z;
        let xx2 = x2 * t.rotation.v.x;
        let xy2 = x2 * t.rotation.v.y;
        let xz2 = x2 * t.rotation.v.z;
        let yy2 = y2 * t.rotation.v.y;
        let yz2 = y2 * t.rotation.v.z;
        let zz2 = z2 * t.rotation.v.z;
        let sy2 = y2 * t.rotation.s;
        let sz2 = z2 * t.rotation.s;
        let sx2 = x2 * t.rotation.s;
        [
            [(1. - yy2 - zz2) * t.scale.x, (     xy2 + sz2) * t.scale.x, (     xz2 - sy2) * t.scale.x, 0.],
            [(     xy2 - sz2) * t.scale.y, (1. - xx2 - zz2) * t.scale.y, (     yz2 + sx2) * t.scale.y, 0.],
            [(     xz2 + sy2) * t.scale.z, (     yz2 - sx2) * t.scale.z, (1. - xx2 - yy2) * t.scale.z, 0.],
            [t.translation.x, t.translation.y, t.translation.z, 1.]
        ]
    }
}
impl Mul<Mat4x4> for Transform {
    type Output = Mat4x4;
    fn mul(self, rhs: Mat4x4) -> Self::Output {
        Mat4x4::from(self) * rhs
    }
}
impl Mul<Vec3> for Transform {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        self.translation + (self.rotation * (rhs * self.scale))
    }
}
impl Mul<Self> for Transform {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Self::new(
            self.rotation * (rhs.translation * self.scale) + self.translation,
            self.rotation * rhs.rotation,
            self.scale * rhs.scale
        )
    }
}