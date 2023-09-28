pub struct Writer(pub Vec<u8>);
#[allow(unused)]
impl Writer {
    #[inline(always)]
    pub fn new(_type: u8) -> Self {
        Self(vec![_type])
    }
    #[inline(always)]
    pub fn write_u8(&mut self, v: u8) {
        self.0.push(v)
    }
    #[inline(always)]
    pub fn write_u32(&mut self, v: u32) {
        self.0.append(&mut v.to_be_bytes().to_vec())
    }
    #[inline(always)]
    pub fn write_vec_f32(&mut self, v: Vec<f32>) {
        self.0.append(&mut (v.len() as u32).to_be_bytes().to_vec());
        self.0.append(&mut v.into_iter().map(|v|v.to_be_bytes()).flatten().collect())
    }
    #[inline(always)]
    pub fn write_vec_u16(&mut self, v: Vec<u16>) {
        self.0.append(&mut (v.len() as u32).to_be_bytes().to_vec());
        self.0.append(&mut v.into_iter().map(|v|v.to_be_bytes()).flatten().collect())
    }
    #[inline(always)]
    pub fn write_vec_u8(&mut self, mut v: Vec<u8>) {
        self.0.append(&mut (v.len() as u32).to_be_bytes().to_vec());
        self.0.append(&mut v)
    }
    #[inline(always)]
    pub fn write_vec_u32(&mut self, v: Vec<u32>) {
        self.0.append(&mut (v.len() as u32).to_be_bytes().to_vec());
        self.0.append(&mut v.into_iter().map(|v|v.to_be_bytes()).flatten().collect())
    }
    #[inline(always)]
    pub fn write_mat4x4(&mut self, v: [[f32;4];4]) {
        self.0.append(&mut v.into_iter().flatten().map(|v|v.to_be_bytes()).flatten().collect())
    }
}