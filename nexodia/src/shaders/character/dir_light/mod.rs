use engine::{vertex::*, Engine, Animator, Light, Object, Material as _};
use wgpu::{Device, BindGroupLayout, BindGroup};

pub fn bgl(device: &Device) -> BindGroupLayout {
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: None,
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None
                },
                count: None
            },
            wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None
                },
                count: None
            }
        ]
    })
}

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct MaterialBinding {
    pub color: [f32;4]
}
pub struct Material {
    bg: BindGroup
}
impl Material {
    pub fn new(e: &Engine, animator: &Animator, light: &Light) -> Self {
        Self {
            bg: e.device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: None,
                layout: &bgl(&e.device),
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: animator.buffer.as_entire_binding()
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: light.buffer.as_entire_binding()
                    }
                ]
            })
        }
    }
}
impl engine::Material for Material {
    fn bind_group(&self) -> &wgpu::BindGroup { &self.bg }
}

shader!(
    @material    Material,
    @vertex      engine::vertex::pnj::Vertex,
    @instance    (),
    @vbls        [],
    @bgls        [bgl],
    @frag_stage  false
);

// impl engine::Shader for Shader {
//     type Material = Material;
//     type Vertex = engine::vertex::pnj::Vertex;
//     type Instance = ();
//     fn pipeline(&self) -> &wgpu::RenderPipeline { &self.0 }
//     fn new(e: &'static engine::Engine) -> Self {
//         let shader = e.device.create_shader_module(wgpu::ShaderModuleDescriptor {
//             label: None,
//             source: wgpu::ShaderSource::Wgsl(std::borrow::Cow::Owned(
//                 include_str!("./shader.wgsl")
//                     .replace("#MAX_JOINTS", &engine::MAX_JOINTS.to_string())
//             ))
//         });
//         let render_pipeline_layout = e.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
//             label: None,
//             bind_group_layouts: &[
//                 &bgl(&e.device)
//             ],
//             push_constant_ranges: &[]
//         });
//         Self(
//             e.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
//                 label: None,
//                 layout: Some(&render_pipeline_layout),
//                 vertex: wgpu::VertexState {
//                     module: &shader,
//                     entry_point: "vs_main",
//                     buffers: &[
//                         Self::Vertex::LAYOUT
//                     ],
//                 },
//                 fragment: None,
//                 primitive: wgpu::PrimitiveState {
//                     topology: wgpu::PrimitiveTopology::TriangleList,
//                     strip_index_format: None,
//                     front_face: wgpu::FrontFace::Ccw,
//                     cull_mode: Some(wgpu::Face::Back),
//                     polygon_mode: wgpu::PolygonMode::Fill,
//                     unclipped_depth: false,
//                     conservative: false
//                 },
//                 depth_stencil: Some(wgpu::DepthStencilState {
//                     format: DEPTH_FORMAT,
//                     depth_write_enabled: true,
//                     depth_compare: wgpu::CompareFunction::Less,
//                     stencil: wgpu::StencilState::default(),
//                     bias: wgpu::DepthBiasState::default()
//                 }),
//                 multisample: wgpu::MultisampleState {
//                     count: 1,
//                     mask: !0,
//                     alpha_to_coverage_enabled: false
//                 },
//                 multiview: None
//             })
//         )
//     }
// }
impl engine::ObjectRenderer for Shader {
    fn render_object<'r, 's: 'r>(&'s self, render_pass: &mut wgpu::RenderPass<'r>, object: &'s Object<Self>) where Self: Sized {
        render_pass.set_pipeline(&self.0);
        render_pass.set_vertex_buffer(0, object.mesh.vertices_buffer.slice(..));
        object.material.set(render_pass, 0);
        render_pass.draw(0..object.mesh.vertices_len, 0..1);
    }
}