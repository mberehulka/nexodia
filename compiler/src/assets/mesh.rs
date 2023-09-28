use std::path::Path;
use gltf::Node;
use math::{Mat4x4, Vec3};
use bincode::{Decode, Encode};

use crate::{Settings, Asset};

#[repr(C)]
#[derive(Clone, Encode, Decode)]
pub struct Joint {
    pub parents: Vec<u8>,
    pub pose: Mat4x4
}

#[repr(C)]
#[derive(Clone, Encode, Decode)]
pub struct Skeleton {
    pub joints: Vec<Joint>
}

#[repr(C)]
#[derive(Encode, Decode)]
pub struct Mesh {
    pub skeleton: Option<Skeleton>,
    pub positions: Vec<[f32;3]>,
    pub uvs: Vec<[f32;2]>,
    pub normals: Vec<[f32;3]>,
    pub joints: Vec<[u8;4]>,
    pub weights: Vec<[f32;4]>,
    pub indices: Vec<u32>
}
impl Asset for Mesh {
    fn compile(path: &Path, settings: &Settings) -> Self {
        let (gltf, buffers, _) = gltf::import(path).unwrap();
        let mesh = gltf.meshes().next().unwrap();
        let primitive = mesh.primitives().next().unwrap();
        let primitive_reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));

        let mut ibms = vec![];
        let skeleton = if settings.skeleton {
            let skin = gltf.skins().next().unwrap();
            let skin_reader = skin.reader(|buffer| Some(&buffers[buffer.index()]));
            ibms = skin_reader.read_inverse_bind_matrices().unwrap()
                .map(|v|Mat4x4::from(v))
                .collect();
            let joints: Vec<Node> = skin.joints().collect();
            Some(Skeleton {
                joints: joints.iter()
                    .zip(ibms.clone())
                    .map(|(joint, ibm)| {
                        let parents = get_gltf_node_parents_id(&joints, &joint);
                        let pose = ibm.inverted().unwrap();
                        Joint {
                            pose: if let Some(parent) = parents.first() {
                                pose * ibms[*parent as usize]
                            } else {
                                pose
                            },
                            parents
                        }
                    })
                    .collect()
            })
        } else { None };

        let joints = if settings.joints {
            primitive_reader.read_joints(0).unwrap().into_u16().map(|v|{
                [v[0] as u8, v[1] as u8, v[2] as u8, v[3] as u8]
            }).collect()
        } else { vec![] };

        let weights = if settings.joints {
            primitive_reader.read_weights(0).unwrap().into_f32().collect()
        } else { vec![] };

        Self {
            skeleton,
            positions: if settings.skeleton_apply_ibm {
                primitive_reader.read_positions().unwrap()
                    .enumerate()
                    .map(|(i, p)| {
                        let v4 = Vec3::from(p).extend();
                        (
                            ((ibms[joints[i][0]as usize] * v4) * weights[i][0]) +
                            ((ibms[joints[i][1]as usize] * v4) * weights[i][1]) +
                            ((ibms[joints[i][2]as usize] * v4) * weights[i][2]) +
                            ((ibms[joints[i][3]as usize] * v4) * weights[i][3])
                        ).truncate().into()
                    })
                    .collect()
            } else {
                primitive_reader.read_positions().unwrap().collect()
            },
            joints,
            weights,
            uvs: if settings.uvs {
                primitive_reader.read_tex_coords(0).unwrap().into_f32().collect()
            } else { vec![] },
            normals: if settings.normals {
                primitive_reader.read_normals().unwrap().collect()
            } else { vec![] },
            indices: primitive_reader.read_indices().unwrap().into_u32().collect()
        }
    }
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