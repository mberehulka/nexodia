pub struct Color([f32;4]);
pub trait ToColor {
    fn to_color(self) -> Color;
}

// Type -> Color
impl From<[f32;4]> for Color {
    fn from(value: [f32;4]) -> Self {
        Self(value)
    }
}
impl ToColor for [f32;4] {
    fn to_color(self) -> Color { self.into() }
}

impl From<[f32;3]> for Color {
    fn from(v: [f32;3]) -> Self {
        Self([v[0], v[1], v[2], 1.])
    }
}
impl ToColor for [f32;3] {
    fn to_color(self) -> Color { self.into() }
}

impl From<&str> for Color {
    fn from(v: &str) -> Self {
        let v = v.strip_prefix("#").unwrap_or(v);
        let b = v.as_bytes();
        match b.len() {
            3 => Self([
                hex_to_f32(b[0], 0),
                hex_to_f32(b[1], 0),
                hex_to_f32(b[2], 0),
                1.
            ]),
            4 => Self([
                hex_to_f32(b[0], 0),
                hex_to_f32(b[1], 0),
                hex_to_f32(b[2], 0),
                hex_to_f32(b[3], 0)
            ]),
            6 => Self([
                hex_to_f32(b[0], b[1]),
                hex_to_f32(b[2], b[3]),
                hex_to_f32(b[4], b[5]),
                1.
            ]),
            8 => Self([
                hex_to_f32(b[0], b[1]),
                hex_to_f32(b[2], b[3]),
                hex_to_f32(b[4], b[5]),
                hex_to_f32(b[6], b[7])
            ]),
            _ => panic!("Bad format")
        }
    }
}
impl ToColor for &str {
    fn to_color(self) -> Color { self.into() }
}

impl From<usize> for Color {
    fn from(v: usize) -> Self {
        Self([
            (v & 0xff0000) as f32 / 0xff as f32,
            (v & 0x00ff00) as f32 / 0xff as f32,
            (v & 0x0000ff) as f32 / 0xff as f32,
            1.
        ])
    }
}
impl ToColor for usize {
    fn to_color(self) -> Color { self.into() }
}

impl From<(u8, u8, u8)> for Color {
    fn from(v: (u8, u8, u8)) -> Self {
        Self([
            v.0 as f32 / 255.,
            v.1 as f32 / 255.,
            v.2 as f32 / 255.,
            1.
        ])
    }
}
impl ToColor for (u8, u8, u8) {
    fn to_color(self) -> Color { self.into() }
}


// Color -> Type
impl From<Color> for [f32;4] {
    fn from(value: Color) -> Self {
        value.0
    }
}
impl From<Color> for [f32;3] {
    fn from(v: Color) -> Self {
        [v.0[0], v.0[1], v.0[2]]
    }
}


#[inline(always)]
const fn byte_to_hex(b: u8) -> u8 {
    if b >= b'0' && b <= b'9' {
        b - b'0'
    } else if b >= b'a' && b <= b'f' {
        b - b'a' + 10
    } else if b >= b'A' && b <= b'F' {
        b - b'A' + 10
    } else {
        panic!("Bad format")
    }
}

#[inline(always)]
pub fn hex_to_f32(a: u8, b: u8) -> f32 {
    ((byte_to_hex(a) << 4) + byte_to_hex(b)) as f32 / 255.
}