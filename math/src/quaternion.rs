use std::ops::{Mul, Sub, Add, Neg};
use bincode::{Decode, Encode};

use crate::{Vec3, Mat4x4, Mat3x3, Vec4};

#[derive(Default, Copy, Clone, Encode, Decode)]
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
    pub const fn from_vs(v: Vec3, s: f32) -> Self {
        Self { v, s }
    }
    #[inline(always)]
    pub fn conjugate(self) -> Self {
        Self::from_vs(-self.v, self.s)
    }
    #[inline(always)]
    pub fn from_euler(x: f32, y: f32, z: f32) -> Self {
        let (sx, cx) = (x * 0.5).sin_cos();
        let (sy, cy) = (y * 0.5).sin_cos();
        let (sz, cz) = (z * 0.5).sin_cos();
        Self::new(
            sx * cy * cz + sy * sz * cx,
           -sx * sz * cy + sy * cx * cz,
            sx * sy * cz + sz * cx * cy,
           -sx * sy * sz + cx * cy * cz
        )
    }
    #[inline(always)]
    pub fn normalize(self) -> Self {
        self * (1. / self.dot(self).sqrt())
    }
    #[inline(always)]
    pub fn dot(self, other: Self) -> f32 {
        self.s * other.s + self.v.dot(other.v)
    }
    #[inline(always)]
    pub fn distance(self, other: Self) -> f32 {
        self.dot(other).sqrt()
    }
    #[inline(always)]
    pub fn lerp(self, mut other: Self, amount: f32) -> Self {
        if self.dot(other) < 0. {
            other = -other
        }
        (self * (1. - amount) + other * amount).normalize()
    }
    #[inline(always)]
    pub fn from_axis_angle(axis: Vec3, angle: f32) -> Self {
        let (s, c) = (angle * 0.5).sin_cos();
        Self::from_vs(axis * s, c)
    }
    #[inline(always)]
    pub fn from_angle_x(angle: f32) -> Self {
        let (s, c) = (angle * 0.5).sin_cos();
        Self::from_vs(Vec3::new(s, 0., 0.), c)
    }
    #[inline(always)]
    pub fn from_angle_y(angle: f32) -> Self {
        let (s, c) = (angle * 0.5).sin_cos();
        Self::from_vs(Vec3::new(0., s, 0.), c)
    }
    #[inline(always)]
    pub fn from_angle_z(angle: f32) -> Self {
        let (s, c) = (angle * 0.5).sin_cos();
        Self::from_vs(Vec3::new(0., 0., s), c)
    }
}
impl From<Vec3> for Quaternion {
    fn from(Vec3 {x, y, z}: Vec3) -> Self {
        Self::from_euler(x, y, z)
    }
}
impl Mul<Vec3> for Quaternion {
    type Output = Vec3;
    fn mul(self, v: Vec3) -> Self::Output {
        self.v.cross(self.v.cross(v) + v * self.s) * 2. + v
    }
}
impl From<Quaternion> for [f32;4] {
    fn from(value: Quaternion) -> Self {
        [
            value.v.x,
            value.v.y,
            value.v.z,
            value.s
        ]
    }
}
impl From<Quaternion> for Mat4x4 {
    fn from(quat: Quaternion) -> Mat4x4 {
        let x2 = quat.v.x + quat.v.x;
        let y2 = quat.v.y + quat.v.y;
        let z2 = quat.v.z + quat.v.z;
        let xx2 = x2 * quat.v.x;
        let xy2 = x2 * quat.v.y;
        let xz2 = x2 * quat.v.z;
        let yy2 = y2 * quat.v.y;
        let yz2 = y2 * quat.v.z;
        let zz2 = z2 * quat.v.z;
        let sy2 = y2 * quat.s;
        let sz2 = z2 * quat.s;
        let sx2 = x2 * quat.s;
        Mat4x4::new(
            Vec4::new(1. - yy2 - zz2, xy2 + sz2, xz2 - sy2, 0.),
            Vec4::new(xy2 - sz2, 1. - xx2 - zz2, yz2 + sx2, 0.),
            Vec4::new(xz2 + sy2, yz2 - sx2, 1. - xx2 - yy2, 0.),
            Vec4::new(0., 0., 0., 1.)
        )
    }
}
impl Mul<Quaternion> for Quaternion {
    type Output = Quaternion;
    fn mul(self, rhs: Quaternion) -> Self::Output {
        Quaternion::new(
            self.s * rhs.v.x + self.v.x * rhs.s   + self.v.y * rhs.v.z - self.v.z * rhs.v.y,
            self.s * rhs.v.y + self.v.y * rhs.s   + self.v.z * rhs.v.x - self.v.x * rhs.v.z,
            self.s * rhs.v.z + self.v.z * rhs.s   + self.v.x * rhs.v.y - self.v.y * rhs.v.x,
            self.s * rhs.s   - self.v.x * rhs.v.x - self.v.y * rhs.v.y - self.v.z * rhs.v.z
        )
    }
}
impl Mul<f32> for Quaternion {
    type Output = Quaternion;
    fn mul(self, rhs: f32) -> Self::Output {
        Quaternion::from_vs(self.v * rhs, self.s * rhs)
    }
}
impl Neg for Quaternion {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self::from_vs(-self.v, -self.s)
    }
}
impl Sub<Self> for Quaternion {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self::from_vs(self.v - rhs.v, self.s  - rhs.s)
    }
}
impl Add<Quaternion> for Quaternion {
    type Output = Quaternion;
    fn add(self, rhs: Quaternion) -> Self::Output {
        Quaternion::from_vs(self.v + rhs.v, self.s  + rhs.s)
    }
}
impl From<Mat3x3> for Quaternion {
    fn from(mat: Mat3x3) -> Quaternion {
        let trace = mat.trace();
        if trace >= 0.{
            let s = (1. + trace).sqrt();
            let w = 0.5 * s;
            let s = 0.5 / s;
            let x = (mat.y.z - mat.z.y) * s;
            let y = (mat.z.x - mat.x.z) * s;
            let z = (mat.x.y - mat.y.x) * s;
            Quaternion::new(x, y, z, w)
        } else if (mat.x.x > mat.y.y) && (mat.x.x > mat.z.z) {
            let s = ((mat.x.x - mat.y.y - mat.z.z) + 1.).sqrt();
            let x = 0.5 * s;
            let s = 0.5 / s;
            let y = (mat.y.x + mat.x.y) * s;
            let z = (mat.x.z + mat.z.x) * s;
            let w = (mat.y.z - mat.z.y) * s;
            Quaternion::new(x, y, z, w)
        } else if mat.y.y > mat.z.z {
            let s = ((mat.y.y - mat.x.x - mat.z.z) + 1.).sqrt();
            let y = 0.5 * s;
            let s = 0.5 / s;
            let z = (mat.z.y + mat.y.z) * s;
            let x = (mat.y.x + mat.x.y) * s;
            let w = (mat.z.x - mat.x.z) * s;
            Quaternion::new(x, y, z, w)
        } else {
            let s = ((mat.z.z - mat.x.x - mat.y.y) + 1.).sqrt();
            let z = 0.5 * s;
            let s = 0.5 / s;
            let x = (mat.x.z + mat.z.x) * s;
            let y = (mat.z.y + mat.y.z) * s;
            let w = (mat.x.y - mat.y.x) * s;
            Quaternion::new(x, y, z, w)
        }
    }
}
impl From<Mat4x4> for Quaternion {
    fn from(value: Mat4x4) -> Self {
        Self::from(Mat3x3::from(value))
    }
}