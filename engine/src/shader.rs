use wgpu::{ShaderModuleDescriptor, RenderPipeline, VertexBufferLayout, BindGroupLayout};

use crate::{Material, vertex::Vertex, Object, MaterialBuffer, Engine, InstanceBinding};

pub trait Shader {
    type Material: Material;
    type MaterialBuffer: MaterialBuffer<Self::Material>;
    type Vertex: Vertex;
    type InstanceBinding: InstanceBinding;
    fn pipeline(&self) -> &wgpu::RenderPipeline;
    fn new(e: &'static Engine) -> Self;
    fn default_pipeline(
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
}

pub trait ObjectRender: Shader {
    fn render_object<'r, 's: 'r>(&'s self, render_pass: &mut wgpu::RenderPass<'r>, object: &'s Object<Self>) where Self: Sized {
        render_pass.set_pipeline(self.pipeline());
        render_pass.set_bind_group(1, object.material_buffer.bind_group(), &[]);
        render_pass.set_vertex_buffer(0, object.mesh.vertices_buffer.slice(..));
        render_pass.draw(0..object.mesh.vertices_len, 0..1);
    }
}