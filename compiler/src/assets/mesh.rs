use std::path::Path;
use gltf::Node;
use math::{Mat4x4, Transform, Vec3};
use bincode::{Decode, Encode};

use crate::{Settings, Asset};

#[derive(Clone, Encode, Decode)]
pub struct Joint {
    pub parents: Vec<u8>,
    pub global_ibm: Mat4x4
}

#[derive(Clone, Encode, Decode)]
pub struct Skeleton {
    pub joints: Vec<Joint>,
    pub root: usize
}

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

        let skeleton = if settings.skeleton {
            let skin = gltf.skins().next().unwrap();
            let skin_reader = skin.reader(|buffer| Some(&buffers[buffer.index()]));
            let ibms = skin_reader.read_inverse_bind_matrices().unwrap()
                .map(|v|Mat4x4::from(v))
                .collect::<Vec<_>>();
            let joints: Vec<Node> = skin.joints().collect();
            let joints = joints.iter()
                .zip(ibms.clone())
                .map(|(joint, global_ibm)| {
                    let parents = get_gltf_node_parents_id(&joints, &joint);
                    Joint {
                        parents,
                        global_ibm
                    }
                })
                .collect::<Vec<_>>();
            Some(Skeleton {
                root: joints.iter().position(|j|j.parents.is_empty()).unwrap(),
                joints
            })
        } else { None };

        let meshes = gltf.meshes().collect::<Vec<_>>();
        let primitives = meshes.iter().map(|mesh| mesh.primitives() ).flatten().collect::<Vec<_>>();
        let readers = primitives.iter()
            .map(|primitive| primitive.reader(|buffer| Some(&buffers[buffer.index()])) )
            .collect::<Vec<_>>();

        let positions = readers.iter()
            .map(|reader| reader.read_positions().unwrap() )
            .flatten()
            .collect();

        let uvs = if settings.uvs {
            readers.iter()
                .map(|reader| reader.read_tex_coords(0).unwrap().into_f32() )
                .flatten()
                .collect()
        } else { vec![] };

        let normals = if settings.normals {
            readers.iter()
                .map(|reader| reader.read_normals().unwrap() )
                .flatten()
                .collect()
        } else { vec![] };

        let joints = if settings.joints {
            readers.iter()
                .map(|reader|
                    reader.read_joints(0).unwrap().into_u16().map(|v|{
                        [v[0] as u8, v[1] as u8, v[2] as u8, v[3] as u8]
                    })
                )
                .flatten()
                .collect()
        } else { vec![] };

        let weights = if settings.joints {
            readers.iter()
                .map(|reader| reader.read_weights(0).unwrap().into_f32() )
                .flatten()
                .collect()
        } else { vec![] };

        let indices = if settings.normals {
            readers.iter()
                .map(|reader| reader.read_indices().unwrap().into_u32() )
                .flatten()
                .collect()
        } else { vec![] };

        Self {
            skeleton,
            positions,
            joints,
            weights,
            uvs,
            normals,
            indices
        }
    }
}
impl Mesh {
    pub fn transform(mut self, transform: Transform) -> Self {
        for (position, normal) in self.positions
            .iter_mut()
            .zip(self.normals.iter_mut())
        {
            *position = (transform * Vec3::from(*position)).into();
            *normal = transform.apply_translation_rotation((*normal).into()).into();
        }
        self
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