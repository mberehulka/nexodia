pub struct Writer(pub Vec<u8>);
impl Writer {
    pub fn new(_type: u8) -> Self {
        Self(vec![_type])
    }
    pub fn write_vec_f32(&mut self, v: Vec<f32>) {
        self.0.append(&mut (v.len() as u32).to_be_bytes().to_vec());
        self.0.append(&mut v.into_iter().map(|v|v.to_be_bytes()).flatten().collect())
    }
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