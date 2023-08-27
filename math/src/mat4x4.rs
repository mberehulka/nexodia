use std::ops::Mul;

use crate::{Vec4, Vec3};

#[repr(C)]
#[derive(Debug, Default, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Mat4x4 {
    pub x: Vec4,
    pub y: Vec4,
    pub z: Vec4,
    pub w: Vec4
}
impl Mat4x4 {
    pub const IDENTITY: Mat4x4 = Mat4x4::new(
        Vec4::new(1., 0., 0., 0.),
        Vec4::new(0., 1., 0., 0.),
        Vec4::new(0., 0., 1., 0.),
        Vec4::new(0., 0., 0., 1.)
    );
    #[inline(always)]
    pub const fn new(x: Vec4, y: Vec4, z: Vec4, w: Vec4) -> Self {
        Self { x, y, z, w }
    }
    #[inline(always)]
    pub fn look_to(eye: Vec3, dir: Vec3) -> Self {
        let f = dir.normalized();
        let s = f.cross(Vec3::new(0., 1., 0.)).normalized();
        let u = s.cross(f);
        Self {
            x: Vec4::new(s.x, u.x, -f.x, 0.),
            y: Vec4::new(s.y, u.y, -f.y, 0.),
            z: Vec4::new(s.z, u.z, -f.z, 0.),
            w: Vec4::new(-eye.dot(s), -eye.dot(u), eye.dot(f), 1.)
        }
    }
    #[inline(always)]
    pub fn look_at(eye: Vec3, center: Vec3) -> Self {
        Self::look_to(eye, center - eye)
    }
    #[inline(always)]
    pub fn perspective(fov: f32, aspect: f32, near: f32, far: f32) -> Self {
        let f = 1. / (fov / 2.).tan();
        Self {
            x: Vec4::new(f / aspect, 0., 0., 0.),
            y: Vec4::new(0., f, 0., 0.),
            z: Vec4::new(0., 0., (far + near) / (near - far), -1.),
            w: Vec4::new(0., 0., (2. * far * near) / (near - far), 0.)
        }
    }
}

impl Mul<Mat4x4> for Mat4x4 {
    type Output = Self;
    fn mul(self, rhs: Mat4x4) -> Self::Output {
        Self::new(
            self.x*rhs.x.x + self.y*rhs.x.y + self.z*rhs.x.z + self.w*rhs.x.w,
            self.x*rhs.y.x + self.y*rhs.y.y + self.z*rhs.y.z + self.w*rhs.y.w,
            self.x*rhs.z.x + self.y*rhs.z.y + self.z*rhs.z.z + self.w*rhs.z.w,
            self.x*rhs.w.x + self.y*rhs.w.y + self.z*rhs.w.z + self.w*rhs.w.w,
        )
    }
}
impl From<[[f32;4];4]> for Mat4x4 {
    fn from(v: [[f32;4];4]) -> Self {
        Self::new(
            Vec4::from(v[0]),
            Vec4::from(v[1]),
            Vec4::from(v[2]),
            Vec4::from(v[3])
        )
    }
}
impl From<Mat4x4> for [[f32;4];4] {
    fn from(v: Mat4x4) -> Self {
        [
            v.x.into(),
            v.y.into(),
            v.z.into(),
            v.w.into()
        ]
    }
}
impl PartialEq<Mat4x4> for Mat4x4 {
    fn eq(&self, other: &Mat4x4) -> bool {
        self.x == other.x &&
        self.y == other.y &&
        self.z == other.z &&
        self.w == other.w
    }
}