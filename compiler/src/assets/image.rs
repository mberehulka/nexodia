use std::{path::Path, io::BufReader, fs::File};
use bincode::{Decode, Encode};

use crate::{Settings, Asset};

#[derive(Encode, Decode)]
pub enum Pixels {
    RGB(Vec<u8>),
    ARGB(Vec<u8>)
}

#[derive(Encode, Decode)]
pub struct Image {
    pub width: u32,
    pub height: u32,
    pub pixels: Pixels
}
impl Image {
    pub fn get_pixels_rgba(self) -> Vec<u8> {
        match self.pixels {
            Pixels::ARGB(v) => v,
            Pixels::RGB(v) => v.chunks(3).map(|v|[v[0], v[1], v[2], 255]).flatten().collect()
        }
    }
}
impl Asset for Image {
    fn compile(path: &Path, settings: &Settings) -> Self {
        let reader = BufReader::new(File::open(path).unwrap());
        let mut img = image::load(reader, image::ImageFormat::from_path(path).unwrap()).unwrap();
        if settings.image_scale != 1. {
            img = img.resize(
                (img.width() as f32 * settings.image_scale)as u32,
                (img.height() as f32 * settings.image_scale)as u32,
                image::imageops::FilterType::Nearest
            )
        }
        Self {
            width: img.width(),
            height: img.height(),
            pixels: if settings.image_opacity {
                Pixels::ARGB(img.as_rgba8().unwrap().to_vec())
            } else {
                Pixels::RGB(img.as_rgb8().unwrap().to_vec())
            }
        }
    }
}