struct Vertex {
    @location(0) position: vec3<f32>,
    @location(1) uv: vec2<f32>
};

struct Camera {
    @location(0) perspective: mat4x4<f32>
};
@group(0) @binding(0)
var<uniform> camera: Camera;

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>
};

@vertex
fn vs_main(vertex: Vertex) -> VertexOutput {
    var vout: VertexOutput;
    vout.position = camera.perspective * vec4<f32>(vertex.position, 1.);
    vout.uv = vertex.uv;
    return vout;
}

@group(1) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(1)@binding(1)
var s_diffuse: sampler;

@fragment
fn fs_main(vout: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(t_diffuse, s_diffuse, vout.uv);
}