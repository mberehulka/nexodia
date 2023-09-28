use crate::Vec3;

pub struct Mat3x3 {
    pub x: Vec3,
    pub y: Vec3,
    pub z: Vec3
}
impl Mat3x3 {
    #[inline(always)]
    pub const fn new(x: Vec3, y: Vec3, z: Vec3) -> Self {
        Self { x, y, z }
    }
    #[inline(always)]
    pub fn determinant(self) -> f32 {
        self.x.x * (self.y.y * self.z.z - self.z.y * self.y.z)
            - self.y.x * (self.x.y * self.z.z - self.z.y * self.x.z)
            + self.z.x * (self.x.y * self.y.z - self.y.y * self.x.z)
    }
}