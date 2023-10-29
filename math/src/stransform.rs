use std::ops::Mul;
use bincode::{Encode, Decode};

use crate::{Vec3, Quaternion, Mat4x4, Vec4};

#[derive(Copy, Clone, Encode, Decode)]
pub struct SimpleTransform {
    pub translation: Vec3,
    pub rotation: Quaternion
}
impl Default for SimpleTransform {
    fn default() -> Self {
        Self {
            translation: Default::default(),
            rotation: Default::default()
        }
    }
}
impl SimpleTransform {
    #[inline(always)]
    pub const fn new(translation: Vec3, rotation: Quaternion) -> Self {
        Self { translation, rotation }
    }
    #[inline(always)]
    pub fn lerp(&mut self, other: Self, amount: f32) {
        self.translation.lerp(other.translation, amount);
        self.rotation = self.rotation.nlerp(other.rotation, amount)
    }
    #[inline(always)]
    pub fn apply_translation_rotation(self, other: Vec3) -> Vec3 {
        self.translation + (self.rotation * other)
    }
}
impl From<SimpleTransform> for Mat4x4 {
    fn from(t: SimpleTransform) -> Mat4x4 {
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
            Vec4::new(1. - yy2 - zz2,      xy2 + sz2,      xz2 - sy2, 0.),
            Vec4::new(     xy2 - sz2, 1. - xx2 - zz2,      yz2 + sx2, 0.),
            Vec4::new(     xz2 + sy2,      yz2 - sx2, 1. - xx2 - yy2, 0.),
            t.translation.extend(1.)
        )
    }
}
impl From<SimpleTransform> for [[f32;4];4] {
    fn from(t: SimpleTransform) -> [[f32;4];4] {
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
            [1. - yy2 - zz2,      xy2 + sz2,      xz2 - sy2, 0.],
            [     xy2 - sz2, 1. - xx2 - zz2,      yz2 + sx2, 0.],
            [     xz2 + sy2,      yz2 - sx2, 1. - xx2 - yy2, 0.],
            [t.translation.x, t.translation.y, t.translation.z, 1.]
        ]
    }
}
impl Mul<Mat4x4> for SimpleTransform {
    type Output = Mat4x4;
    fn mul(self, rhs: Mat4x4) -> Self::Output {
        Mat4x4::from(self) * rhs
    }
}
impl Mul<Vec3> for SimpleTransform {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        self.translation + (self.rotation * rhs)
    }
}
impl Mul<Self> for SimpleTransform {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Self::new(
            self.rotation * rhs.translation + self.translation,
            self.rotation * rhs.rotation
        )
    }
}