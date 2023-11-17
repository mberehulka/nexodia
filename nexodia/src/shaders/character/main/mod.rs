use engine::{Engine, Animator, Light, utils::ToColor};
use wgpu::BufferUsages;

bind_group_layouts!(
    Uniform(FRAGMENT)
    Uniform(VERTEX_FRAGMENT)
    Uniform(FRAGMENT)
    TextureView(FRAGMENT, Depth)
    TextureSampler(FRAGMENT, Filtering)
);

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct MaterialBinding {
    pub color: [f32;4]
}
basic_material!(
    (e: &Engine, animator: &Animator, light: &Light, color: impl ToColor) {
        create_bind_group!(
            bind_group_layouts(&e.device)
            e.new_buffer(
                bytemuck::bytes_of(&MaterialBinding {
                    color: color.to_color().into()
                }),
                BufferUsages::UNIFORM
            ).as_entire_binding()
            animator.buffer.as_entire_binding()
            light.buffer.as_entire_binding()
            wgpu::BindingResource::TextureView(&light.depth_texture.view)
            wgpu::BindingResource::Sampler(&light.depth_texture.sampler)
        )
    }
    bind_group_index 1
);

shader!(
    material    Material
    vertex      engine::vertex::pnj::Vertex
    instance    ()
    vbls        [Self::Vertex::LAYOUT]
    bgls        [&e.camera_buffer.bgl, &bind_group_layouts(&e.device)]
    frag_stage  true
);
impl engine::ObjectRenderer for Shader {}