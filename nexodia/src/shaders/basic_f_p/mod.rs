use engine::Vertex;

uniform_buffer!(
    color: [f32;4]
);

shader!(
    Material, engine::vertex::p::Vertex, (),
    [],
    [engine::utils::bgls::uniform]
);
impl engine::ObjectRenderer for Shader {}