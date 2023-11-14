struct Vertex {
    @location(0) position: vec3<f32>,
    @location(1) uv: vec2<f32>
};

struct Light {
    @location(0) perspective: mat4x4<f32>,
    @location(1) direction: vec4<f32>
};
@group(0) @binding(0)
var<uniform> light: Light;

@vertex
fn vs_main(vertex: Vertex) -> @builtin(position) vec4<f32> {
    return light.perspective * vec4<f32>(vertex.position, 1.);
}