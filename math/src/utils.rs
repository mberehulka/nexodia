#[inline(always)]
pub fn deg_to_rad(deg: f32) -> f32 {
    deg * (std::f32::consts::PI / 180.)
}
#[inline(always)]
pub fn rad_to_deg(rad: f32) -> f32 {
    rad * (180. / std::f32::consts::PI)
}