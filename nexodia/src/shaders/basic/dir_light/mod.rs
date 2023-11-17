use engine::{Engine, Light};

bind_group_layouts!(
    Uniform(VERTEX)
);

basic_material!(
    (e: &Engine, light: &Light) {
        create_bind_group!(
            bind_group_layouts(&e.device)
            light.buffer.as_entire_binding()
        )
    }
    bind_group_index 0
);

shader!(
    material    Material
    vertex      engine::vertex::pu::Vertex
    instance    ()
    vbls        [Self::Vertex::LAYOUT]
    bgls        [&bind_group_layouts(&e.device)]
    frag_stage  false
);
impl engine::ObjectRenderer for Shader {}