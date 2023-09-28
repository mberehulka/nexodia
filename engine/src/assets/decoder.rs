use std::path::Path;
use bincode::{config, Decode};

pub fn decode<T: Decode>(path: impl AsRef<Path>) -> T {
    let bytes = zstd::stream::decode_all(std::fs::OpenOptions::new().read(true).open(&path).unwrap()).unwrap();
    bincode::decode_from_slice(&bytes[..], config::standard()).unwrap().0
}