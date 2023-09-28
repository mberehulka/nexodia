use std::{path::Path, sync::Arc, time::Instant, marker::PhantomData};
use math::Mat4x4;
use wgpu::util::DeviceExt;

use crate::{Engine, Reader, Vertex};


#[derive(Clone)]
pub struct SkeletonJoint {
    pub parents: Vec<u8>,
    pub tpose: Mat4x4,
    pub ibm: Mat4x4
}

#[derive(Clone)]
pub struct Skeleton {
    pub joints: Vec<SkeletonJoint>
}

#[derive(Clone)]
pub struct Mesh<V: Vertex> {
    vertex_type: PhantomData<V>,
    pub vertices_buffer: Arc<wgpu::Buffer>,
    pub vertices_len: u32,
    pub skeleton: Option<Skeleton>
}
impl Engine {
    pub fn load_mesh<V: Vertex>(&self, path: impl AsRef<Path>) -> Mesh<V> {
        let start = Instant::now();
        
        let mut r = Reader::new(path.as_ref());
        assert!(r.read_u8() == b'M');

        let skeleton = if r.read_u8() == 1 {
            let joints_count = r.read_u8() as usize;
            let mut joints = Vec::with_capacity(joints_count);
            for _ in 0..joints_count {
                joints.push(SkeletonJoint {
                    parents: r.read_vec_u8(),
                    tpose: r.read_mat4x4().into(),
                    ibm: r.read_mat4x4().into()
                })
            }
            Some(Skeleton {
                joints
            })
        } else { None };

        let positions = r.read_vec_f32()
            .chunks(3)
            .into_iter()
            .map(|v|[v[0], v[1], v[2]])
            .collect::<Vec<_>>();

        let has_uvs = r.read_u8() == 1;
        let uvs = if has_uvs {
            r.read_vec_f32()
                .chunks(2)
                .into_iter()
                .map(|v|[v[0], v[1]])
                .collect::<Vec<_>>()
        } else { vec![] };

        let has_normals = r.read_u8() == 1;
        let normals = if has_normals {
            r.read_vec_f32()
                .chunks(3)
                .into_iter()
                .map(|v|[v[0], v[1], v[2]])
                .collect::<Vec<_>>()
        } else { vec![] };
        
        let has_joints = r.read_u8() == 1;
        let joints = if has_joints {
            r.read_vec_u8()
                .chunks(4)
                .into_iter()
                .map(|v|[v[0], v[1], v[2], v[3]])
                .collect::<Vec<_>>()
        } else { vec![] };
        let weights = if has_joints {
            r.read_vec_f32()
                .chunks(4)
                .into_iter()
                .map(|v|[v[0], v[1], v[2], v[3]])
                .collect::<Vec<_>>()
        } else { vec![] };

        let indices = r.read_vec_u32();
        let vertices_len = indices.len() as u32;

        assert!(V::requires(has_uvs, has_normals), "Mesh '{}' is not compatible with this Vertex", path.as_ref().display());
        
        let mut contents = Vec::with_capacity(indices.len() * bytemuck::bytes_of(&V::default()).len());
        for i in indices {
            contents.append(&mut bytemuck::bytes_of(&V::new(i as usize, &positions, &uvs, &normals, &joints, &weights)).to_vec())
        }

        let vertices_buffer = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: &contents,
            usage: wgpu::BufferUsages::VERTEX
        });

        info!("Mesh '{}' loaded in: {}ms", path.as_ref().display(), (Instant::now() - start).as_millis());
        
        Mesh {
            vertex_type: PhantomData::default(),
            vertices_buffer: vertices_buffer.into(),
            vertices_len,
            skeleton
        }
    }
}