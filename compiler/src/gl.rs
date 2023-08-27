use std::path::Path;

use crate::{settings::Settings, writer::Writer};

pub fn compile(path: &Path, settings: &Settings) -> Vec<u8> {
    let mut w = Writer::new(b'M');
    let (gltf, buffers, _) = gltf::import(path).unwrap();
    let mesh = gltf.meshes().next().unwrap();
    let primitive = mesh.primitives().next().unwrap();
    let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));

    w.write_vec_f32(reader.read_positions().unwrap().flatten().collect());

    w.write_byte(settings.uv as u8);
    if settings.uv { w.write_vec_f32(reader.read_tex_coords(0).unwrap().into_f32().flatten().collect()) }
    
    w.write_byte(settings.normal as u8);
    if settings.normal { w.write_vec_f32(reader.read_normals().unwrap().flatten().collect()) }

    w.write_vec_u32_compact(reader.read_indices().unwrap().into_u32().collect());

    w.0
}