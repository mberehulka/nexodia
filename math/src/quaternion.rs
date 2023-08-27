use std::ops::Mul;

use crate::{Vec3, MutVec3, MutF32};

#[derive(Default, Clone, Copy)]
pub struct Quaternion {
    pub v: Vec3,
    pub s: f32
}
impl Quaternion {
    #[inline(always)]
    pub const fn new(x: f32, y: f32, z: f32, s: f32) -> Self {
        Self {
            v: Vec3::new(x, y, z),
            s
        }
    }
    #[inline(always)]
    pub const fn from_sv(v: Vec3, s: f32) -> Self {
        Self { v, s }
    }
    #[inline(always)]
    pub fn conjugate(self) -> Self {
        Self::from_sv(-self.v, self.s)
    }
    #[inline(always)]
    pub fn from_euler(x: f32, y: f32, z: f32) -> Self {
        let x = x * 0.5;
        let y = y * 0.5;
        let z = z * 0.5;
        let sx = x.sin();
        let cx = x.cos();
        let sy = y.sin();
        let cy = y.cos();
        let sz = z.sin();
        let cz = z.cos();
        Self::new(
            -sx * sy * sz + cx * cy * cz,
             sx * cy * cz + sy * sz * cx,
            -sx * sz * cy + sy * cx * cz,
             sx * sy * cz + sz * cx * cy,
        )
    }
}
impl From<Vec3> for Quaternion {
    fn from(v: Vec3) -> Self {
        Self::from_euler(v.x, v.y, v.z)
    }
}
impl Mul<Vec3> for Quaternion {
    type Output = Vec3;
    fn mul(self, v: Vec3) -> Self::Output {
        self.v.cross(self.v.cross(v) + v * self.s) * 2. + v
    }
}

#[derive(Default)]
pub struct MutQuaternion {
    pub v: MutVec3,
    pub s: MutF32
}
impl MutQuaternion {
    #[inline(always)]
    pub const fn new(x: f32, y: f32, z: f32, s: f32) -> Self {
        Self {
            v: MutVec3::new(x, y, z),
            s: MutF32::new(s)
        }
    }
    #[inline(always)]
    pub fn from_sv(v: Vec3, s: f32) -> Self {
        Self {
            v: v.into(),
            s: s.into()
        }
    }
    #[inline(always)]
    pub fn conjugate(self) -> Self {
        Self::from_sv(-self.v.get(), self.s.get())
    }
    #[inline(always)]
    pub fn from_euler(x: f32, y: f32, z: f32) -> Self {
        let x = x * 0.5;
        let y = y * 0.5;
        let z = z * 0.5;
        let sx = x.sin();
        let cx = x.cos();
        let sy = y.sin();
        let cy = y.cos();
        let sz = z.sin();
        let cz = z.cos();
        Self::new(
            -sx * sy * sz + cx * cy * cz,
             sx * cy * cz + sy * sz * cx,
            -sx * sz * cy + sy * cx * cz,
             sx * sy * cz + sz * cx * cy,
        )
    }
}
impl From<Vec3> for MutQuaternion {
    fn from(v: Vec3) -> Self {
        Self::from_euler(v.x, v.y, v.z)
    }
}
impl Mul<Vec3> for MutQuaternion {
    type Output = Vec3;
    fn mul(self, v: Vec3) -> Self::Output {
        let sv = self.v.get();
        sv.cross(sv.cross(v) + v * self.s.get()) * 2. + v
    }
}