use std::{path::Path, io::Cursor};
use image::io::Reader;

use crate::{settings::Settings, writer::Writer};

pub fn compile(path: &Path, settings: &Settings) -> Vec<u8> {
    let mut w = Writer::new(b'I');
    w.write_byte(settings.pixel_type as u8);
    let img = Cursor::new(std::fs::read(path).unwrap());
    let img = Reader::new(img).with_guessed_format().unwrap().decode().unwrap();
    w.write_u32(img.width());
    w.write_u32(img.height());
    w.0.append(&mut if settings.pixel_type {
        img.into_rgba8().to_vec()
    } else {
        img.into_rgb8().to_vec()
    });
    w.0
}