struct Vertex {
    @location(0) position: vec3<f32>,
    @location(1) uv: vec2<f32>
};

// Camera
    struct Camera {
        @location(0) perspective: mat4x4<f32>
    };
    @group(0) @binding(0)
    var<uniform> camera: Camera;

// Material
    @group(1) @binding(0)
    var t_diffuse: texture_2d<f32>;
    @group(1)@binding(1)
    var s_diffuse: sampler;

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
    @location(1) uv: vec2<f32>
};

@vertex
fn vs_main(vertex: Vertex) -> VertexOutput {
    var vout: VertexOutput;
    vout.position = vec4<f32>(vertex.position, 1.);
    vout.clip_position = camera.perspective * vout.position;
    vout.uv = vertex.uv;
    return vout;
}

@fragment
fn fs_main(vin: VertexOutput) -> @location(0) vec4<f32> {
    let uv = textureSample(t_diffuse, s_diffuse, vin.uv);

    let position = light.perspective * vin.position;
    let light_texture_depth = textureSample(light_shadow_texture, light_shadow_sampler, vec2<f32>(
        position.x * 0.5 + 0.5,
        position.y * -0.5 + 0.5
    ));
    var light_shadow = 1.0;
    if (light_texture_depth < position.z - 0.01) {
        light_shadow = 0.5;
    }

    return uv * light_shadow;
}