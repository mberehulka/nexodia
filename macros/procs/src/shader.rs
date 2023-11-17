use proc_macro::TokenStream;
use quote::quote;
use syn::{parse::ParseStream, parse_macro_input};

use crate::utils::{AnyToString, assert_ident};

pub struct Args {
    pub material: AnyToString,
    pub vertex: AnyToString,
    pub instance: AnyToString,
    pub vbls: AnyToString,
    pub bgls: AnyToString,
    pub frag_stage: AnyToString
}
impl syn::parse::Parse for Args {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        assert_ident(input, "material")?;
        let material = input.parse::<AnyToString>()?;

        assert_ident(input, "vertex")?;
        let vertex = input.parse::<AnyToString>()?;

        assert_ident(input, "instance")?;
        let instance = input.parse::<AnyToString>()?;

        assert_ident(input, "vbls")?;
        let vbls = input.parse::<AnyToString>()?;

        assert_ident(input, "bgls")?;
        let bgls = input.parse::<AnyToString>()?;

        assert_ident(input, "frag_stage")?;
        let frag_stage = input.parse::<AnyToString>()?;

        Ok(Self {
            material,
            vertex,
            instance,
            vbls,
            bgls,
            frag_stage
        })
    }
}

pub fn shader(inp: TokenStream) -> TokenStream {
    let Args {
        material,
        vertex,
        instance,
        vbls,
        bgls,
        frag_stage,
    } = parse_macro_input!(inp as Args);
    quote!(
        use wgpu::{ShaderModuleDescriptor, RenderPipeline, VertexBufferLayout, BindGroupLayout};
        use engine::{Vertex};
        pub struct Shader(wgpu::RenderPipeline);
        impl engine::Shader for Shader {
            type Material = #material;
            type Vertex = #vertex;
            type Instance = #instance;
            fn pipeline(&self) -> &wgpu::RenderPipeline { &self.0 }
            fn new(e: &'static engine::Engine) -> Self {
                let shader = e.device.create_shader_module(wgpu::ShaderModuleDescriptor {
                    label: Some(module_path!()),
                    source: wgpu::ShaderSource::Wgsl(std::borrow::Cow::Owned(
                        include_str!("./shader.wgsl")
                            .replace("#MAX_JOINTS", &engine::MAX_JOINTS.to_string())
                    ))
                });
                let render_pipeline_layout = e.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some(module_path!()),
                    bind_group_layouts: &#bgls,
                    push_constant_ranges: &[]
                });
                let targets = &[Some(wgpu::ColorTargetState {
                    format: e.surface_config.lock().unwrap().format,
                    blend: None,
                    write_mask: wgpu::ColorWrites::COLOR
                })];
                Self (
                    e.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                        label: Some(module_path!()),
                        layout: Some(&render_pipeline_layout),
                        vertex: wgpu::VertexState {
                            module: &shader,
                            entry_point: "vs_main",
                            buffers: &#vbls
                        },
                        fragment: if #frag_stage {
                            Some(wgpu::FragmentState {
                                module: &shader,
                                entry_point: "fs_main",
                                targets
                            })
                        } else {
                            None
                        },
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
                            format: engine::DEPTH_FORMAT,
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
                )
            }
        }
    ).into()
}