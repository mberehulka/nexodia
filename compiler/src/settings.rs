use std::path::Path;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

#[derive(Default, Clone, Serialize, Deserialize, Debug)]
pub struct Settings {
    pub compression_level: i32,
    pub uvs: bool,
    pub normals: bool,
    pub joints: bool,
    pub skeleton: bool,
    pub skeleton_apply_ibm: bool,
    pub image_opacity: bool,
    pub image_scale: f32
}
impl Settings {
    pub fn merge(&mut self, path: &Path) {
        if let Ok(bytes) = std::fs::read(path.join("settings.json")) {
            let new: Value = serde_json::from_slice(&bytes).unwrap();
            let mut s = json!(&self);
            let s = s.as_object_mut().unwrap();
            for (name, value) in new.as_object().unwrap() {
                s.insert(name.clone(), value.clone());
            }
            *self = serde_json::from_value(json!(s)).unwrap()
        }
    }
}