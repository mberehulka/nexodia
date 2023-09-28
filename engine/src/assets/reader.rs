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
    pub fn get_rest(mut self) -> Vec<u8> {
        let i = self.cursor;
        self.cursor = self.bytes.len();
        self.bytes[i..].to_vec()
    }
    #[inline(always)]
    pub fn read_u8(&mut self) -> u8 {
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
    #[inline(always)]
    pub fn read_vec_f32(&mut self) -> Vec<f32> {
        let size = self.read_u32() as usize * 4;
        self.cursor += size;
        self.bytes[self.cursor-size..self.cursor]
            .chunks_exact(4)
            .into_iter()
            .map(|v|f32::from_be_bytes([v[0], v[1], v[2], v[3]]))
            .collect()
    }
    #[inline(always)]
    pub fn read_vec_u8(&mut self) -> Vec<u8> {
        let size = self.read_u32() as usize;
        self.cursor += size;
        self.bytes[self.cursor-size..self.cursor].to_vec()
    }
    #[inline(always)]
    pub fn read_vec_u16(&mut self) -> Vec<u16> {
        let size = self.read_u32() as usize * 2;
        self.cursor += size;
        self.bytes[self.cursor-size..self.cursor]
            .chunks_exact(2)
            .into_iter()
            .map(|v|u16::from_be_bytes([v[0], v[1]]))
            .collect()
    }
    #[inline(always)]
    pub fn read_vec_u32(&mut self) -> Vec<u32> {
        let size = self.read_u32() as usize * 4;
        self.cursor += size;
        self.bytes[self.cursor-size..self.cursor]
            .chunks_exact(4)
            .into_iter()
            .map(|v|u32::from_be_bytes([v[0], v[1], v[2], v[3]]))
            .collect()
    }
    #[inline(always)]
    pub fn read_mat4x4(&mut self) -> [[f32;4];4] {
        let i = self.cursor;
        self.cursor += 64;
        [
            [
                f32::from_be_bytes([self.bytes[i   ], self.bytes[i+1 ], self.bytes[i+2 ], self.bytes[i+3 ]]),
                f32::from_be_bytes([self.bytes[i+4 ], self.bytes[i+5 ], self.bytes[i+6 ], self.bytes[i+7 ]]),
                f32::from_be_bytes([self.bytes[i+8 ], self.bytes[i+9 ], self.bytes[i+10], self.bytes[i+11]]),
                f32::from_be_bytes([self.bytes[i+12], self.bytes[i+13], self.bytes[i+14], self.bytes[i+15]])
            ],
            [
                f32::from_be_bytes([self.bytes[i+16], self.bytes[i+17], self.bytes[i+18], self.bytes[i+19]]),
                f32::from_be_bytes([self.bytes[i+20], self.bytes[i+21], self.bytes[i+22], self.bytes[i+23]]),
                f32::from_be_bytes([self.bytes[i+24], self.bytes[i+25], self.bytes[i+26], self.bytes[i+27]]),
                f32::from_be_bytes([self.bytes[i+28], self.bytes[i+29], self.bytes[i+30], self.bytes[i+31]])
            ],
            [
                f32::from_be_bytes([self.bytes[i+32], self.bytes[i+33], self.bytes[i+34], self.bytes[i+35]]),
                f32::from_be_bytes([self.bytes[i+36], self.bytes[i+37], self.bytes[i+38], self.bytes[i+39]]),
                f32::from_be_bytes([self.bytes[i+40], self.bytes[i+41], self.bytes[i+42], self.bytes[i+43]]),
                f32::from_be_bytes([self.bytes[i+44], self.bytes[i+45], self.bytes[i+46], self.bytes[i+47]])
            ],
            [
                f32::from_be_bytes([self.bytes[i+48], self.bytes[i+49], self.bytes[i+50], self.bytes[i+51]]),
                f32::from_be_bytes([self.bytes[i+52], self.bytes[i+53], self.bytes[i+54], self.bytes[i+55]]),
                f32::from_be_bytes([self.bytes[i+56], self.bytes[i+57], self.bytes[i+58], self.bytes[i+59]]),
                f32::from_be_bytes([self.bytes[i+60], self.bytes[i+61], self.bytes[i+62], self.bytes[i+63]])
            ]
        ]
    }
    #[inline(always)]
    pub fn read_mat3x3(&mut self) -> [[f32;3];3] {
        let i = self.cursor;
        self.cursor += 36;
        [
            [
                f32::from_be_bytes([self.bytes[i   ], self.bytes[i+1 ], self.bytes[i+2 ], self.bytes[i+3 ]]),
                f32::from_be_bytes([self.bytes[i+4 ], self.bytes[i+5 ], self.bytes[i+6 ], self.bytes[i+7 ]]),
                f32::from_be_bytes([self.bytes[i+8 ], self.bytes[i+9 ], self.bytes[i+10], self.bytes[i+11]])
            ],
            [
                f32::from_be_bytes([self.bytes[i+12], self.bytes[i+13], self.bytes[i+14], self.bytes[i+15]]),
                f32::from_be_bytes([self.bytes[i+16], self.bytes[i+17], self.bytes[i+18], self.bytes[i+19]]),
                f32::from_be_bytes([self.bytes[i+20], self.bytes[i+21], self.bytes[i+22], self.bytes[i+23]])
            ],
            [
                f32::from_be_bytes([self.bytes[i+24], self.bytes[i+25], self.bytes[i+26], self.bytes[i+27]]),
                f32::from_be_bytes([self.bytes[i+28], self.bytes[i+29], self.bytes[i+30], self.bytes[i+31]]),
                f32::from_be_bytes([self.bytes[i+32], self.bytes[i+33], self.bytes[i+34], self.bytes[i+35]])
            ]
        ]
    }
    #[inline(always)]
    pub fn read_str(&mut self) -> String {
        let mut res = String::new();
        loop {
            let c = self.read_u8();
            res.push(c as char);
            if c == b'#' { break }
        }
        res
    }
    #[inline(always)]
    pub fn is_finished(&self) -> bool {
        self.cursor == self.bytes.len()
    }
}
impl Drop for Reader {
    fn drop(&mut self) {
        assert!(self.is_finished(), "Cursor at: {}, end: {}", self.cursor, self.bytes.len())
    }
}