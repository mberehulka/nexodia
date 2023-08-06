struct Vertex {
    @location(0) position: vec3<f32>
};

struct Camera {
    @location(0) perspective: mat4x4<f32>
};
@group(0) @binding(0)
var<uniform> camera: Camera;

@vertex
fn vs_main(vertex: Vertex) -> @builtin(position) vec4<f32> {
    return camera.perspective * vec4<f32>(vertex.position, 1.);
}

struct Material {
    @location(0) color: vec4<f32>
}
@group(1) @binding(0)
var<uniform> material: Material;

@fragment
fn fs_main() -> @location(0) vec4<f32> {
    return material.color;
}