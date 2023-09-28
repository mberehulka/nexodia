use std::path::Path;
use bincode::{config, Decode};

mod texture;    pub use texture::*;
mod mesh;       pub use mesh::*;
mod reader;     pub use reader::*;
mod animation;  pub use animation::*;
pub mod vertex;   pub use vertex::Vertex;

pub fn decode<T: Decode>(path: impl AsRef<Path>) -> T {
    let bytes = zstd::stream::decode_all(std::fs::OpenOptions::new().read(true).open(&path).unwrap()).unwrap();
    bincode::decode_from_slice(&bytes[..], config::standard()).unwrap().0
}