use engine::{Light, Texture, Engine};

bind_group_layouts!(
    TextureView(FRAGMENT, Float { filterable: true })
    TextureSampler(FRAGMENT, Filtering)
    Uniform(FRAGMENT)
    TextureView(FRAGMENT, Depth)
    TextureSampler(FRAGMENT, Filtering)
);

basic_material!(
    (e: &Engine, light: &Light, texture: Texture) {
        create_bind_group!(
            bind_group_layouts(&e.device)
            wgpu::BindingResource::TextureView(&texture.view)
            wgpu::BindingResource::Sampler(&texture.sampler)
            light.buffer.as_entire_binding()
            wgpu::BindingResource::TextureView(&light.depth_texture.view)
            wgpu::BindingResource::Sampler(&light.depth_texture.sampler)
        )
    }
    bind_group_index 1
);

shader!(
    material    Material
    vertex      engine::vertex::pu::Vertex
    instance    ()
    vbls        [Self::Vertex::LAYOUT]
    bgls        [&e.camera_buffer.bgl, &bind_group_layouts(&e.device)]
    frag_stage  true
);
impl engine::ObjectRenderer for Shader {}