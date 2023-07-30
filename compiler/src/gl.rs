use std::{path::PathBuf, io::Write};

use crate::settings::Settings;

/// u8 -> M (Mesh)
/// str -> vertex_type
/// u8 -> indexed (0 or 1)
/// if indexed {
///     u32 -> position values
///     [[f32, f32, f32]; position values]
/// 
///     u32 -> index values
///     u8 -> index type (A: u8, B: u16, C: u32)
///     [index type; index values]
/// } else {
///     u32 -> index values
///     [
///         [f32, f32, f32] (positions)
///         ; index values
///     ]
/// }

pub fn compile(path: PathBuf, settings: &Settings) -> Vec<u8> {
    let mut b = Vec::new();
    writeln!(b, "M{}#{}",
        settings.vertex_type,
        settings.indexed as u8
    ).unwrap();
    let (gltf, buffers, _) = gltf::import(&path).unwrap();
    for mesh in gltf.meshes() {
        for primitive in mesh.primitives() {
            let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));
            if settings.indexed {
                let positions = reader.read_positions().unwrap().collect::<Vec<_>>();
                b.write(&(positions.len()as u32).to_be_bytes()).unwrap();
                for [x, y, z] in positions {
                    b.write(&x.to_be_bytes()).unwrap();
                    b.write(&y.to_be_bytes()).unwrap();
                    b.write(&z.to_be_bytes()).unwrap();
                }

                let indices = reader.read_indices().unwrap().into_u32().collect::<Vec<_>>();
                b.write(&(indices.len()as u32).to_be_bytes()).unwrap();
                let max_index = *indices.iter().max().unwrap_or(&0);
                if max_index <= u8::MAX as u32 {
                    b.write(b"A").unwrap();
                } else if max_index <= u16::MAX as u32 {
                    b.write(b"B").unwrap();
                } else {
                    b.write(b"C").unwrap();
                }
                for i in indices {
                    if max_index <= u8::MAX as u32 {
                        b.write(&[i as u8]).unwrap();
                    } else if max_index <= u16::MAX as u32 {
                        b.write(&(i as u16).to_be_bytes()).unwrap();
                    } else {
                        b.write(&i.to_be_bytes()).unwrap();
                    }
                }
            } else {
                let positions = reader.read_positions().unwrap().collect::<Vec<_>>();
                let indices = reader.read_indices().unwrap().into_u32().collect::<Vec<_>>();
                b.write(&(indices.len()as u32).to_be_bytes()).unwrap();
                for i in indices {
                    let [x, y, z] = positions[i as usize];
                    b.write(&x.to_be_bytes()).unwrap();
                    b.write(&y.to_be_bytes()).unwrap();
                    b.write(&z.to_be_bytes()).unwrap();
                }
            }
        }
    }
    b
}