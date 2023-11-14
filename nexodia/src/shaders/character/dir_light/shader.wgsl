struct Vertex {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) joints: vec4<u32>,
    @location(3) weights: vec4<f32>
};

struct Animator {
    @location(0) joint: array<mat4x4<f32>, #MAX_JOINTS>
};
@group(0) @binding(0)
var<uniform> animator: Animator;

struct Light {
    @location(0) perspective: mat4x4<f32>,
    @location(1) direction: vec4<f32>
};
@group(0) @binding(1)
var<uniform> light: Light;

@vertex
fn vs_main(vertex: Vertex) -> @builtin(position) vec4<f32> {
    let v4 = vec4<f32>(vertex.position, 1.);
    return light.perspective * (
        ((animator.joint[vertex.joints[0]] * v4) * vertex.weights[0]) +
        ((animator.joint[vertex.joints[1]] * v4) * vertex.weights[1]) +
        ((animator.joint[vertex.joints[2]] * v4) * vertex.weights[2]) +
        ((animator.joint[vertex.joints[3]] * v4) * vertex.weights[3])
    );
}