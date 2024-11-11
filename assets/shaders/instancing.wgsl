#import bevy_pbr::mesh_functions::{get_world_from_local, mesh_position_local_to_clip}

struct Vertex {
    @builtin(vertex_index) index: u32,
    @location(0) i_pos_scale: vec4<f32>,
    @location(1) i_color: vec4<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
};

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    var out: VertexOutput;

    var mesh = array<vec3<f32>, 4>(
        vec3<f32>(0.5, 0.5, 0.0),
        vec3<f32>(0.0, 0.5, 0.0),
        vec3<f32>(0.5, 0.0, 0.0),
        vec3<f32>(0.0, 0.0, 0.0)
    );

    let p = mesh[vertex.index];
    let position = p * vertex.i_pos_scale.w + vertex.i_pos_scale.xyz;
    out.clip_position = mesh_position_local_to_clip(
        mat4x4<f32>(
            vec4(1.0, 0.0, 0.0, 0.0),
            vec4(0.0, 1.0, 0.0, 0.0),
            vec4(0.0, 0.0, 1.0, 0.0),
            vec4(0.0, 0.0, 0.0, 1.0)
        ),
        vec4<f32>(position, 1.0)
    );
    out.color = vertex.i_color;
    return out;
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    return in.color;
}