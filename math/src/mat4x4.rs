use std::ops::Mul;
use bincode::{Decode, Encode};

use crate::{Vec4, Vec3, Mat3x3, det_sub_proc_unsafe};

#[derive(Debug, Default, Copy, Clone, Encode, Decode)]
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
    #[inline(always)]
    pub fn transposed(self) -> Self {
        Self {
            x: Vec4::new(self.x.x, self.y.x, self.z.x, self.w.x),
            y: Vec4::new(self.x.y, self.y.y, self.z.y, self.w.y),
            z: Vec4::new(self.x.z, self.y.z, self.z.z, self.w.z),
            w: Vec4::new(self.x.w, self.y.w, self.z.w, self.w.w)
        }
    }
    #[inline(always)]
    pub fn determinant(self) -> f32 {
        unsafe { det_sub_proc_unsafe(self, 1, 2, 3) }
            .dot(Vec4::new(self.x.x, self.y.x, self.z.x, self.w.x))
    }
    #[inline(always)]
    pub fn inverted(self) -> Option<Self> {
        let det = self.determinant();
        if det == 0. {
            None
        } else {
            let inv_det = 1. / det;
            let t = self.transposed();
            let cf = |i, j| {
                let mat = match i {
                    0 => Mat3x3::new(t.y.truncate_n(j), t.z.truncate_n(j), t.w.truncate_n(j)),
                    1 => Mat3x3::new(t.x.truncate_n(j), t.z.truncate_n(j), t.w.truncate_n(j)),
                    2 => Mat3x3::new(t.x.truncate_n(j), t.y.truncate_n(j), t.w.truncate_n(j)),
                    3 => Mat3x3::new(t.x.truncate_n(j), t.y.truncate_n(j), t.z.truncate_n(j)),
                    _ => panic!("out of range")
                };
                let sign = if (i + j) & 1 == 1 { -1. } else { 1. };
                mat.determinant() * sign * inv_det
            };
            Some(Mat4x4::from([
                [cf(0, 0), cf(0, 1), cf(0, 2), cf(0, 3)],
                [cf(1, 0), cf(1, 1), cf(1, 2), cf(1, 3)],
                [cf(2, 0), cf(2, 1), cf(2, 2), cf(2, 3)],
                [cf(3, 0), cf(3, 1), cf(3, 2), cf(3, 3)]
            ]))
        }
    }
    #[inline(always)]
    pub const fn from_translation(v: Vec3) -> Self {
        Self::new(
            Vec4::new(1., 0., 0., v.x),
            Vec4::new(0., 1., 0., v.y),
            Vec4::new(0., 0., 1., v.z),
            Vec4::new(0., 0., 0., 1.)
        )
    }
    #[inline(always)]
    pub const fn from_scale(v: Vec3) -> Self {
        Self::new(
            Vec4::new(v.x, 0., 0., 0.),
            Vec4::new(0., v.y, 0., 0.),
            Vec4::new(0., 0., v.z, 0.),
            Vec4::new(0., 0., 0., 1.)
        )
    }
}

impl Mul<Mat4x4> for Mat4x4 {
    type Output = Self;
    fn mul(self, rhs: Mat4x4) -> Self::Output {
        Self::new(
            self.x*rhs.x.x + self.y*rhs.x.y + self.z*rhs.x.z + self.w*rhs.x.w,
            self.x*rhs.y.x + self.y*rhs.y.y + self.z*rhs.y.z + self.w*rhs.y.w,
            self.x*rhs.z.x + self.y*rhs.z.y + self.z*rhs.z.z + self.w*rhs.z.w,
            self.x*rhs.w.x + self.y*rhs.w.y + self.z*rhs.w.z + self.w*rhs.w.w
        )
    }
}
impl From<Mat4x4> for [f32;16] {
    fn from(v: Mat4x4) -> Self {
        [
            v.x.x, v.x.y, v.x.z, v.x.w,
            v.y.x, v.y.y, v.y.z, v.y.w,
            v.z.x, v.z.y, v.z.z, v.z.w,
            v.w.x, v.w.y, v.w.z, v.w.w
        ]
    }
}
impl From<[[f32;4];4]> for Mat4x4 {
    fn from(v: [[f32;4];4]) -> Self {
        Self::new(
            unsafe { *v.get_unchecked(0) }.into(),
            unsafe { *v.get_unchecked(1) }.into(),
            unsafe { *v.get_unchecked(2) }.into(),
            unsafe { *v.get_unchecked(3) }.into()
        )
    }
}
impl From<Mat4x4> for [[f32;4];4] {
    fn from(v: Mat4x4) -> Self {
        [
            [v.x.x, v.x.y, v.x.z, v.x.w],
            [v.y.x, v.y.y, v.y.z, v.y.w],
            [v.z.x, v.z.y, v.z.z, v.z.w],
            [v.w.x, v.w.y, v.w.z, v.w.w]
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

impl Mul<Vec4> for Mat4x4 {
    type Output = Vec4;
    fn mul(self, rhs: Vec4) -> Self::Output {
        self.x * rhs.x +
        self.y * rhs.y +
        self.z * rhs.z +
        self.w * rhs.w
    }
}
impl From<Mat4x4> for Mat3x3 {
    fn from(value: Mat4x4) -> Self {
        Self::new(value.x.truncate(), value.y.truncate(), value.z.truncate())
    }
}
impl Mul<Vec3> for Mat4x4 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
		let w = rhs.x * self.x.w + rhs.y * self.y.w + rhs.z * self.z.w + self.w.w;
		if w == 0. {
            Vec3::new(
                rhs.x * self.x.x + rhs.y * self.y.x + rhs.z * self.z.x + self.w.x,
                rhs.x * self.x.y + rhs.y * self.y.y + rhs.z * self.z.y + self.w.y,
                rhs.x * self.x.z + rhs.y * self.y.z + rhs.z * self.z.z + self.w.z
            )
		} else {
            Vec3::new(
                (rhs.x * self.x.x + rhs.y * self.y.x + rhs.z * self.z.x + self.w.x) / w,
                (rhs.x * self.x.y + rhs.y * self.y.y + rhs.z * self.z.y + self.w.y) / w,
                (rhs.x * self.x.z + rhs.y * self.y.z + rhs.z * self.z.z + self.w.z) / w
            )
        }
    }
}