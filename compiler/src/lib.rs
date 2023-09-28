use std::path::Path;
use bincode::{config, Decode, Encode};

mod assets;    pub use assets::*;
mod settings;  pub use settings::*;

pub trait Asset: Encode + Decode {
    fn compile(path: &Path, settings: &Settings) -> Self;
    fn bytes(self) -> Vec<u8> {
        bincode::encode_to_vec(&self, config::standard()).unwrap()
    }
}