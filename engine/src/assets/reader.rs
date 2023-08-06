use std::path::Path;

pub struct Reader {
    bytes: Vec<u8>,
    cursor: usize
}
impl Reader {
    pub fn new(path: impl AsRef<Path>) -> Self {
        Self {
            bytes: zstd::decode_all(
                std::fs::OpenOptions::new()
                    .read(true)
                    .open(path.as_ref()).unwrap()
            ).unwrap(),
            cursor: 0
        }
    }
    #[inline(always)]
    pub fn read_byte(&mut self) -> u8 {
        self.cursor += 1;
        self.bytes[self.cursor-1]
    }
    #[inline(always)]
    pub fn read_u32(&mut self) -> u32 {
        self.cursor += 4;
        u32::from_be_bytes([
            self.bytes[self.cursor-4],
            self.bytes[self.cursor-3],
            self.bytes[self.cursor-2],
            self.bytes[self.cursor-1]
        ])
    }
    pub fn read_vec_f32(&mut self) -> Vec<f32> {
        let size = self.read_u32() as usize * 4;
        self.cursor += size;
        self.bytes[self.cursor-size..self.cursor]
            .chunks_exact(4)
            .into_iter()
            .map(|v|f32::from_be_bytes([v[0], v[1], v[2], v[3]]))
            .collect()
    }
    pub fn read_vec_u32_compact(&mut self) -> Vec<u32> {
        let t = self.read_byte() as usize;
        let size = self.read_u32() as usize * (t/8);
        self.cursor += size;
        match t {
            8 => self.bytes[self.cursor-size..self.cursor]
                .into_iter()
                .map(|v|*v as u32)
                .collect(),
            16 => self.bytes[self.cursor-size..self.cursor]
                .chunks_exact(2)
                .into_iter()
                .map(|v|u16::from_be_bytes([v[0], v[1]]) as u32)
                .collect(),
            32 => self.bytes[self.cursor-size..self.cursor]
                .chunks_exact(4)
                .into_iter()
                .map(|v|u32::from_be_bytes([v[0], v[1], v[2], v[3]]))
                .collect(),
            _ => unreachable!()
        }
    }
}