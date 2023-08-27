use std::path::Path;

use crate::Reader;

#[derive(Clone)]
pub struct Image {
    pub pixel_opacity: bool,
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<u8>
}
impl Image {
    pub fn new(path: impl AsRef<Path>) -> Image {
        let mut r = Reader::new(path);
        assert!(r.read_byte() == b'I');
        let pixel_opacity = r.read_byte() == 1;
        let width = r.read_u32();
        let height = r.read_u32();
        let pixels = if pixel_opacity {
            r.get_rest()
        } else {
            let mut res = Vec::with_capacity((width * height * 4)as usize);
            for v in r.get_rest().chunks(3) {
                res.push(v[0]);
                res.push(v[1]);
                res.push(v[2]);
                res.push(255)
            }
            res
        };
        assert!(pixels.len() == (width * height * 4)as usize);
        Image {
            pixel_opacity,
            width,
            height,
            pixels
        }
    }
}