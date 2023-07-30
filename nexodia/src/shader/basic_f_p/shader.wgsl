struct Vertex {
    @location(0) position: vec3<f32>
};
struct Transform {
    @location(1) position: vec3<f32>,
    @location(2) scale: vec3<f32>
};

struct Material {
    @location(0) color: vec3<f32>
};
@group(0) @binding(0)
var<uniform> camera: Camera;

@vertex
fn vs_main(vertex: Vertex, transform: Transform) -> @builtin(position) vec4<f32> {
    return camera.perspective * vec4<f32>((transform.scale * vertex.position) + transform.position, 1.0);
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