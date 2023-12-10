#import bevy_pbr::{
    mesh_functions,     skinning,
    view_transformations::position_world_to_clip,
}

//     morph::morph,
//     forward_io::{Vertex},
//     
// }
#import bevy_render::instance_index::get_instance_index
// #import bevy_pbr::mesh_functions::{get_model_matrix, mesh_position_local_to_clip}

#import bevy_pbr::forward_io::{Vertex}
// #import bevy_pbr::forward_io::{VertexOutput}
// we can import items from shader modules in the assets folder with a quoted path
// #import "shaders/custom_material_import.wgsl"::COLOR_MULTIPLIER

@group(2) @binding(0) var<uniform> my_material_color: vec4<f32>;
// @group(2) @binding(2) var material_color_texture: texture_2d<f32>;
// @group(2) @binding(3) var material_color_sampler: sampler;



struct VertexOutput {
    // This is `clip position` when the struct is used as a vertex stage output
    // and `frag coord` when used as a fragment stage input
    @builtin(position) position: vec4<f32>,
    @location(0) world_position: vec4<f32>,
    @location(1) world_normal: vec3<f32>,
#ifdef VERTEX_UVS
    @location(2) uv: vec2<f32>,
#endif
#ifdef VERTEX_TANGENTS
    @location(3) world_tangent: vec4<f32>,
#endif

    @location(4) color: vec4<f32>,

#ifdef VERTEX_OUTPUT_INSTANCE_INDEX
    @location(5) @interpolate(flat) instance_index: u32,
#endif
    @location(6) local_position: vec3<f32>,
}


@vertex
fn vertex(vertex_no_morph: Vertex) -> VertexOutput {
    var out: VertexOutput;

#ifdef MORPH_TARGETS
    var vertex = morph_vertex(vertex_no_morph);
#else
    var vertex = vertex_no_morph;
#endif

#ifdef SKINNED
    var model = skinning::skin_model(vertex.joint_indices, vertex.joint_weights);
#else
    // Use vertex_no_morph.instance_index instead of vertex.instance_index to work around a wgpu dx12 bug.
    // See https://github.com/gfx-rs/naga/issues/2416 .
    var model = mesh_functions::get_model_matrix(vertex_no_morph.instance_index);
#endif

#ifdef VERTEX_NORMALS
#ifdef SKINNED
    out.world_normal = skinning::skin_normals(model, vertex.normal);
#else
    out.world_normal = mesh_functions::mesh_normal_local_to_world(
        vertex.normal,
        // Use vertex_no_morph.instance_index instead of vertex.instance_index to work around a wgpu dx12 bug.
        // See https://github.com/gfx-rs/naga/issues/2416
        get_instance_index(vertex_no_morph.instance_index)
    );
#endif
#endif

#ifdef VERTEX_POSITIONS
    out.world_position = mesh_functions::mesh_position_local_to_world(model, vec4<f32>(vertex.position, 1.0));
    out.position = position_world_to_clip(out.world_position.xyz);
#endif

#ifdef VERTEX_UVS
    out.uv = vertex.uv + vec2<f32>(0.5, 0.0);
#endif

#ifdef VERTEX_TANGENTS
    out.world_tangent = mesh_functions::mesh_tangent_local_to_world(
        model,
        vertex.tangent,
        // Use vertex_no_morph.instance_index instead of vertex.instance_index to work around a wgpu dx12 bug.
        // See https://github.com/gfx-rs/naga/issues/2416
        get_instance_index(vertex_no_morph.instance_index)
    );
#endif

#ifdef VERTEX_COLORS
    out.color = vertex.color;
#endif

#ifdef VERTEX_OUTPUT_INSTANCE_INDEX
    // Use vertex_no_morph.instance_index instead of vertex.instance_index to work around a wgpu dx12 bug.
    // See https://github.com/gfx-rs/naga/issues/2416
    out.instance_index = get_instance_index(vertex_no_morph.instance_index);
#endif

#ifdef BASE_INSTANCE_WORKAROUND
    // Hack: this ensures the push constant is always used, which works around this issue:
    // https://github.com/bevyengine/bevy/issues/10509
    // This can be removed when wgpu 0.19 is released
    out.position.x += min(f32(get_instance_index(0u)), 0.0);
#endif

    out.local_position = vertex.position;
    // out.color = material_color;

    return out;
}


@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {

    return vec4<f32>(0.3, 0.8, 0.4, 0.2);
    // var r2 = in.local_position.x*in.local_position.x + in.local_position.z*in.local_position.z;

    // if r2 >= 0.9 && abs(in.local_position.x) < 0.6 && in.local_position.z < 0.0 {
        
    //     // return material_color;
    //     // return my_material_color;
    // }
    // // return vec4<f32>(in.uv.x, in.uv.y, 0.0, 1.0);
    // return vec4<f32>(0.0, 0.0, 0.0, 0.0);

    // return in.color + in.clip_position.x;
    // return vec4<f32>(1.0, 0.3, 0.0, 1.0);
}
