struct Vertex {
    @location(0) position: vec3<f32>,
    @location(1) uv: vec2<f32>,
    @location(2) joints: vec4<u32>,
    @location(3) weights: vec4<f32>
};
struct Instance {
    @location(4) translation: vec3<f32>,
    @location(5) scale: vec3<f32>,
    @location(6) texture_id: u32
};

struct Camera {
    @location(0) perspective: mat4x4<f32>
};
@group(0) @binding(0)
var<uniform> camera: Camera;

struct Joints {
    @location(0) joints: array<mat4x4<f32>, 128>
};
@group(1) @binding(0)
var<uniform> joints: Joints;

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) texture_id: u32
};

fn apply_skin(vertex: Vertex, v3: vec3<f32>) -> vec3<f32> {
    let v4 = vec4<f32>(v3, 1.0);
    return (
        ((joints.joints[vertex.joints[0]] * v4) * vertex.weights[0]) +
        ((joints.joints[vertex.joints[1]] * v4) * vertex.weights[1]) +
        ((joints.joints[vertex.joints[2]] * v4) * vertex.weights[2]) +
        ((joints.joints[vertex.joints[3]] * v4) * vertex.weights[3])
    ).xyz;
}

@vertex
fn vs_main(vertex: Vertex, instance: Instance) -> VertexOutput {
    var vout: VertexOutput;
    vout.position = camera.perspective * vec4<f32>(apply_skin(vertex, vertex.position) * instance.scale + instance.translation, 1.);
    vout.uv = vertex.uv;
    vout.texture_id = instance.texture_id;
    return vout;
}

@group(1) @binding(1)
var t_diffuse: binding_array<texture_2d<f32>>;
@group(1) @binding(2)
var s_diffuse: binding_array<sampler>;

@fragment
fn fs_main(vout: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(t_diffuse[vout.texture_id], s_diffuse[vout.texture_id], vout.uv);
}