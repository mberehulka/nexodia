struct Vertex {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) joints: vec4<u32>,
    @location(3) weights: vec4<f32>
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
struct Light {
    @location(0) dir: vec3<f32>
};
@group(1) @binding(1)
var<uniform> light: Light;

struct VertexOutput {
    @builtin(position) position: vec4<f32>
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
fn vs_main(vertex: Vertex) -> VertexOutput {
    var vout: VertexOutput;
    vout.position = camera.perspective * vec4<f32>(apply_skin(vertex, vertex.position), 1.);
    return vout;
}

@fragment
fn fs_main(vout: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(1.);
}