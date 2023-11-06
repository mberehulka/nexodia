use wgpu::{ShaderModuleDescriptor, RenderPipeline, VertexBufferLayout, BindGroupLayout};

use crate::Engine;

pub fn default_pipeline(
    e: &'static Engine,
    shader_module_desc: ShaderModuleDescriptor,
    buffers: &[VertexBufferLayout],
    bind_group_layouts: &[&BindGroupLayout]
) -> RenderPipeline {
    let shader = e.device.create_shader_module(shader_module_desc);
    let render_pipeline_layout = e.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: None,
        bind_group_layouts,
        push_constant_ranges: &[]
    });
    e.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: None,
        layout: Some(&render_pipeline_layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: "vs_main",
            buffers
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: "fs_main",
            targets: &[Some(wgpu::ColorTargetState {
                format: e.surface_config.lock().unwrap().format,
                blend: None,
                write_mask: wgpu::ColorWrites::COLOR
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
            format: crate::DEPTH_FORMAT,
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
    
#[macro_export]
macro_rules! shaders {
    ($($name: ident),*) => {
        use engine::Shader;
        $( pub mod $name; )*
        pub struct Shaders {
            $( pub $name: $name::Shader ),*
        }
        impl Shaders {
            pub fn new(e: &'static engine::Engine) -> Self {
                let start = std::time::Instant::now();
                let s = Self {
                    $( $name: $name::Shader::new(e) ),*
                };
                info!("Shaders compiled in: {}ms", (std::time::Instant::now() - start).as_millis());
                s
            }
        }
    };
}
pub use shaders;

#[macro_export]
macro_rules! shader {
    (
        $material: ty, $vertex: ty, $instance: ty,
        [$($vbs: expr),*],
        [$($bgls: expr),*]
    ) => {
        pub struct Shader(wgpu::RenderPipeline);
        impl engine::Shader for Shader {
            type Material = $material;
            type Vertex = $vertex;
            type Instance = $instance;
            fn pipeline(&self) -> &wgpu::RenderPipeline { &self.0 }
            fn new(e: &'static engine::Engine) -> Self {
                Self(engine::utils::shaders::default_pipeline(
                    e,
                    wgpu::ShaderModuleDescriptor {
                        label: None,
                        source: wgpu::ShaderSource::Wgsl(std::borrow::Cow::Owned(
                            include_str!("./shader.wgsl")
                                .replace("#MAX_JOINTS", &engine::MAX_JOINTS.to_string())
                        ))
                    },
                    &[
                        Self::Vertex::LAYOUT,
                        $($vbs),*
                    ],
                    &[
                        &e.camera_buffer.bgl,
                        $(&$bgls(&e.device)),*
                    ]
                ))
            }
        }
    };
}
pub use shader;