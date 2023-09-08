struct Vertex {
    @location(0) position: vec3<f32>,
    @location(1) uv: vec2<f32>
};
struct Instance {
    @location(2) translation: vec3<f32>,
    @location(3) scale: vec3<f32>,
    @location(4) texture_id: u32
};

struct Camera {
    @location(0) perspective: mat4x4<f32>
};
@group(0) @binding(0)
var<uniform> camera: Camera;

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) texture_id: u32
};

@vertex
fn vs_main(vertex: Vertex, instance: Instance) -> VertexOutput {
    var vout: VertexOutput;
    vout.position = camera.perspective * vec4<f32>(vertex.position * instance.scale + instance.translation, 1.);
    vout.uv = vertex.uv;
    vout.texture_id = instance.texture_id;
    return vout;
}

@group(1) @binding(0)
var t_diffuse: binding_array<texture_2d<f32>>;
@group(1) @binding(1)
var s_diffuse: binding_array<sampler>;

@fragment
fn fs_main(vout: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(t_diffuse[vout.texture_id], s_diffuse[vout.texture_id], vout.uv);
}