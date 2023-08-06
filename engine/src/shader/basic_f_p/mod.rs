/// Basic Fixed Object with vertex::P

use wgpu::{RenderPipeline, Device, TextureFormat};

use crate::{material_bgl, Engine, camera_bgl};

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Material {
    pub color: [f32;4]
}
impl crate::Material for Material {
    fn cast_slice(&self) -> Vec<u8> {
        bytemuck::cast_slice(&[*self]).to_vec()
    }
    fn shader(&self, e: &'static Engine) -> &'static RenderPipeline {
        &e.shaders.basic_f_p
    }
}

pub fn new(
    device: &Device,
    surface_texture_format: TextureFormat
) -> RenderPipeline {
    let shader = device.create_shader_module(wgpu::include_wgsl!("./shader.wgsl").into());
    let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: None,
        bind_group_layouts: &[
            &camera_bgl(device),
            &material_bgl(device)
        ],
        push_constant_ranges: &[]
    });
    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: None,
        layout: Some(&render_pipeline_layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: "vs_main",
            buffers: &[
                crate::assets::vertex::P::LAYOUT
            ]
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: "fs_main",
            targets: &[Some(wgpu::ColorTargetState {
                format: surface_texture_format,
                blend: None,
                write_mask: wgpu::ColorWrites::ALL
            })]
        }),
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            strip_index_format: None,
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: Some(wgpu::Face::Back),
            polygon_mode: wgpu::PolygonMode::Fill,
            unclipped_depth: false,
            conservative: false
        },
        depth_stencil: Some(wgpu::DepthStencilState {
            format: crate::assets::DEPTH_FORMAT,
            depth_write_enabled: true,
            depth_compare: wgpu::CompareFunction::Less,
            stencil: wgpu::StencilState::default(),
            bias: wgpu::DepthBiasState::default()
        }),
        multisample: wgpu::MultisampleState {
            count: 1,
            mask: !0,
            alpha_to_coverage_enabled: false
        },
        multiview: None
    })
}