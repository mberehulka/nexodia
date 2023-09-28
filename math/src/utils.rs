use crate::{Mat4x4, Vec4};

#[inline(always)]
pub fn deg_to_rad(deg: f32) -> f32 {
    deg * (std::f32::consts::PI / 180.)
}
#[inline(always)]
pub fn rad_to_deg(rad: f32) -> f32 {
    rad * (180. / std::f32::consts::PI)
}

#[inline(always)]
pub unsafe fn det_sub_proc_unsafe(m: Mat4x4, x: usize, y: usize, z: usize) -> Vec4 {
    let s: [f32; 16] = m.into();
    let a = Vec4::new(*s.get_unchecked(4  + x), *s.get_unchecked(12 + x), *s.get_unchecked(     x), *s.get_unchecked(8 + x));
    let b = Vec4::new(*s.get_unchecked(8  + y), *s.get_unchecked(8  + y), *s.get_unchecked(4  + y), *s.get_unchecked(4 + y));
    let c = Vec4::new(*s.get_unchecked(12 + z), *s.get_unchecked(     z), *s.get_unchecked(12 + z), *s.get_unchecked(    z));
    let d = Vec4::new(*s.get_unchecked(8  + x), *s.get_unchecked(8  + x), *s.get_unchecked(4  + x), *s.get_unchecked(4 + x));
    let e = Vec4::new(*s.get_unchecked(12 + y), *s.get_unchecked(     y), *s.get_unchecked(12 + y), *s.get_unchecked(    y));
    let f = Vec4::new(*s.get_unchecked(4  + z), *s.get_unchecked(12 + z), *s.get_unchecked(     z), *s.get_unchecked(8 + z));
    let g = Vec4::new(*s.get_unchecked(12 + x), *s.get_unchecked(     x), *s.get_unchecked(12 + x), *s.get_unchecked(    x));
    let h = Vec4::new(*s.get_unchecked(4  + y), *s.get_unchecked(12 + y), *s.get_unchecked(     y), *s.get_unchecked(8 + y));
    let i = Vec4::new(*s.get_unchecked(8  + z), *s.get_unchecked(8  + z), *s.get_unchecked(4  + z), *s.get_unchecked(4 + z));
    let mut tmp = a.mul_element_wise(b.mul_element_wise(c));
    tmp += d.mul_element_wise(e.mul_element_wise(f));
    tmp += g.mul_element_wise(h.mul_element_wise(i));
    tmp -= a.mul_element_wise(e.mul_element_wise(i));
    tmp -= d.mul_element_wise(h.mul_element_wise(c));
    tmp -= g.mul_element_wise(b.mul_element_wise(f));
    tmp
}