use std::path::Path;
use gltf::Node;
use math::{Mat4x4, Vec3};

use crate::{settings::Settings, writer::Writer};

#[derive(Default)]
pub struct Mesh {
    pub test: Vec<u8>
}
unsafe fn any_as_u8_slice<T: Sized>(p: &T) -> &[u8] {
    core::slice::from_raw_parts((p as *const T) as *const u8, core::mem::size_of::<T>())
}

fn get_gltf_node_parent_id(joints: &Vec<gltf::Node>, j: &gltf::Node) -> Option<u8> {
    for (parent_id, joint) in joints.iter().enumerate() {
        for child in joint.children() {
            if child.index() == j.index() {
                return Some(parent_id as u8)
            }
        }
    }
    None
}
fn get_gltf_node_parents_id(joints: &Vec<gltf::Node>, j: &gltf::Node) -> Vec<u8> {
    let mut res = Vec::new();
    let mut id = get_gltf_node_parent_id(joints, j);
    while id.is_some() {
        res.push(id.unwrap());
        id = get_gltf_node_parent_id(joints, &joints[id.unwrap()as usize])
    }
    res
}

pub fn compile(path: &Path, settings: &Settings) -> Vec<u8> {
    let mut w = Writer::new(b'M');
    let (gltf, buffers, _) = gltf::import(path).unwrap();
    let mesh = gltf.meshes().next().unwrap();
    let primitive = mesh.primitives().next().unwrap();
    let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));

    let res = Mesh::default();
    let b = unsafe { any_as_u8_slice(&res) };

    let skin = if settings.skeleton { gltf.skins().next() } else { None };
    w.write_u8(skin.is_some() as u8);
    let mut skin_joints = Vec::new();
    if let Some(skin) = skin {
        let reader = skin.reader(|buffer| Some(&buffers[buffer.index()]));
        let ibms: Vec<[[f32; 4]; 4]> = reader.read_inverse_bind_matrices().unwrap().collect();
        let joints: Vec<Node> = skin.joints().collect();
        assert!(ibms.len() == joints.len());
        w.write_u8(joints.len() as u8);
        for (i, joint) in joints.iter().enumerate() {
            w.write_vec_u8(get_gltf_node_parents_id(&joints, &joint));
            w.write_mat4x4(Mat4x4::from(ibms[i]).inverted().unwrap_or_default().into());
            skin_joints.push(Mat4x4::from(ibms[i]));
            w.write_mat4x4(ibms[i])
        }
    }

    let (joints, weights) = if settings.joint {
        (
            reader.read_joints(0).unwrap().into_u16().flatten().map(|v|v as u8).collect(),
            reader.read_weights(0).unwrap().into_f32().flatten().collect()
        )
    } else {
        (vec![], vec![])
    };
    
    w.write_vec_f32(reader.read_positions().unwrap().enumerate().map(|(i, p)| {
        let mut p = Vec3::from(p);
        if settings.skelton_apply_ibm {
            p = (
                skin_joints[joints[i]as usize] * p.extend()
            ).truncate()
        }
        <[f32;3]>::from(p)
    }).flatten().collect());

    w.write_u8(settings.uv as u8);
    if settings.uv { w.write_vec_f32(reader.read_tex_coords(0).unwrap().into_f32().flatten().collect()) }
    
    w.write_u8(settings.normal as u8);
    if settings.normal { w.write_vec_f32(reader.read_normals().unwrap().flatten().collect()) }

    w.write_u8(settings.joint as u8);
    if settings.joint {
        w.write_vec_u8(joints);
        w.write_vec_f32(weights)
    }

    w.write_vec_u32(reader.read_indices().unwrap().into_u32().collect());

    w.0
}