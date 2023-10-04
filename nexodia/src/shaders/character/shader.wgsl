struct Vertex {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) joints: vec4<u32>,
    @location(3) weights: vec4<f32>
};

struct Camera {
    @location(0) perspective: mat4x4<f32>,
    @location(1) position: vec4<f32>
};
@group(0) @binding(0)
var<uniform> camera: Camera;

struct Joints {
    @location(0) joint: array<mat4x4<f32>, #MAX_JOINTS>
};
@group(1) @binding(0)
var<uniform> joints: Joints;

struct Light {
    @location(0) direction: vec4<f32>
};
@group(1) @binding(1)
var<uniform> light: Light;

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) normal: vec3<f32>,
    @location(1) joints: vec4<u32>,
    @location(2) weights: vec4<f32>
};

@vertex
fn vs_main(vertex: Vertex) -> VertexOutput {
    var vout: VertexOutput;
    let v4 = vec4<f32>(vertex.position, 1.);
    vout.position = camera.perspective * (
        ((joints.joint[vertex.joints[0]] * v4) * vertex.weights[0]) +
        ((joints.joint[vertex.joints[1]] * v4) * vertex.weights[1]) +
        ((joints.joint[vertex.joints[2]] * v4) * vertex.weights[2]) +
        ((joints.joint[vertex.joints[3]] * v4) * vertex.weights[3])
    );
    vout.normal = vertex.normal;
    vout.joints = vertex.joints;
    vout.weights = vertex.weights;
    return vout;
}

fn mat4x4_to_mat3x3(in: mat4x4<f32>) -> mat3x3<f32> {
    return mat3x3<f32>(in.x.xyz, in.y.xyz, in.z.xyz);
}

@fragment
fn fs_main(vout: VertexOutput) -> @location(0) vec4<f32> {
    let normal = (
        ((mat4x4_to_mat3x3(joints.joint[vout.joints[0]]) * vout.normal) * vout.weights[0]) +
        ((mat4x4_to_mat3x3(joints.joint[vout.joints[1]]) * vout.normal) * vout.weights[1]) +
        ((mat4x4_to_mat3x3(joints.joint[vout.joints[2]]) * vout.normal) * vout.weights[2]) +
        ((mat4x4_to_mat3x3(joints.joint[vout.joints[3]]) * vout.normal) * vout.weights[3])
    );
    return vec4<f32>(vec3<f32>(dot(normal, light.direction.xyz)), 1.);
}