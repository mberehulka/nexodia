pub struct Writer(pub Vec<u8>);
#[allow(unused)]
impl Writer {
    #[inline(always)]
    pub fn new(_type: u8) -> Self {
        Self(vec![_type])
    }
    #[inline(always)]
    pub fn write_byte(&mut self, v: u8) {
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
    pub fn write_vec_u8(&mut self, mut v: Vec<u8>) {
        self.0.append(&mut (v.len() as u32).to_be_bytes().to_vec());
        self.0.append(&mut v)
    }
    #[inline(always)]
    pub fn write_vec_u32_compact(&mut self, v: Vec<u32>) {
        let t = *v.iter().max().unwrap_or(&0);
        self.0.push(
            if t <= u8::MAX as u32 { 8 }
            else if t <= u16::MAX as u32 { 16 }
            else { 32 }
        );
        self.0.append(&mut (v.len() as u32).to_be_bytes().to_vec());
        if t <= u8::MAX as u32 {
            self.0.append(&mut v.into_iter().map(|v|v as u8).collect())
        } else if t <= u16::MAX as u32 {
            self.0.append(&mut v.into_iter().map(|v|(v as u16).to_be_bytes()).flatten().collect())
        } else {
            self.0.append(&mut v.into_iter().map(|v|v.to_be_bytes()).flatten().collect())
        }
    }
}