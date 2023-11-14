struct Vertex {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) joints: vec4<u32>,
    @location(3) weights: vec4<f32>
};

// Camera
    struct Camera {
        @location(0) perspective: mat4x4<f32>,
        @location(1) position: vec4<f32>
    };
    @group(0) @binding(0)
    var<uniform> camera: Camera;

// Material
    struct Material {
        @location(0) color: vec4<f32>
    };
    @group(1) @binding(0)
    var<uniform> material: Material;

    // Animator
        struct Animator {
            @location(0) joint: array<mat4x4<f32>, #MAX_JOINTS>
        };
        @group(1) @binding(1)
        var<uniform> animator: Animator;

    // Light
        struct Light {
            @location(0) perspective: mat4x4<f32>,
            @location(1) direction: vec4<f32>
        };
        @group(1) @binding(2)
        var<uniform> light: Light;
        @group(1) @binding(3)
        var light_shadow_texture: texture_depth_2d;
        @group(1)@binding(4)
        var light_shadow_sampler: sampler;

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) position: vec4<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) @interpolate(flat) joints: vec4<u32>,
    @location(3) @interpolate(flat) weights: vec4<f32>
};

@vertex
fn vs_main(vertex: Vertex) -> VertexOutput {
    var vout: VertexOutput;
    let v4 = vec4<f32>(vertex.position, 1.);
    vout.position = (
        ((animator.joint[vertex.joints[0]] * v4) * vertex.weights[0]) +
        ((animator.joint[vertex.joints[1]] * v4) * vertex.weights[1]) +
        ((animator.joint[vertex.joints[2]] * v4) * vertex.weights[2]) +
        ((animator.joint[vertex.joints[3]] * v4) * vertex.weights[3])
    );
    vout.clip_position = camera.perspective * vout.position;
    vout.normal = vertex.normal;
    vout.joints = vertex.joints;
    vout.weights = vertex.weights;
    return vout;
}

fn mat4x4_to_mat3x3(in: mat4x4<f32>) -> mat3x3<f32> {
    return mat3x3<f32>(in.x.xyz, in.y.xyz, in.z.xyz);
}

@fragment
fn fs_main(vin: VertexOutput) -> @location(0) vec4<f32> {
    let position = light.perspective * vin.position;
    let light_texture_depth = textureSample(light_shadow_texture, light_shadow_sampler, vec2<f32>(
        position.x * 0.5 + 0.5,
        position.y * -0.5 + 0.5
    ));
    var light_shadow = 1.0;
    if (light_texture_depth < position.z - 0.01) {
        light_shadow = 0.5;
    }
    let normal = (
        ((mat4x4_to_mat3x3(animator.joint[vin.joints[0]]) * vin.normal) * vin.weights[0]) +
        ((mat4x4_to_mat3x3(animator.joint[vin.joints[1]]) * vin.normal) * vin.weights[1]) +
        ((mat4x4_to_mat3x3(animator.joint[vin.joints[2]]) * vin.normal) * vin.weights[2]) +
        ((mat4x4_to_mat3x3(animator.joint[vin.joints[3]]) * vin.normal) * vin.weights[3])
    );
    let normal_shadow = dot(normal, light.direction.xyz) * 0.5 + 0.5;
    return vec4<f32>(material.color.xyz * light_shadow, 1.);
}