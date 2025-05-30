# set examples \
#     ssr \
#     lines \
#     ui_material \
#     tonemapping \
#     shader_defs \
#     clustered_decals \
#     gpu_readback \
#     array_texture \
#     storage_buffer \
#     irradiance_volumes \
#     animate_shader \
#     fallback_image \
#     shader_prepass \
#     shader_material \
#     extended_material \
#     custom_phase_item \
#     shader_material_2d \
#     custom_render_phase \
#     automatic_instancing \
#     shader_material_wesl \
#     texture_binding_array \
#     custom_post_processing \
#     custom_vertex_attribute \
#     custom_gltf_vertex_attribute \
#     custom_shader_instancing \
#     shader_material_bindless \
#     specialized_mesh_pipeline \
#     extended_material_bindless \
#     shader_material_screenspace_texture

set examples \
    ssr \
    clustered_decals \
    array_texture \
    irradiance_volumes \
    fallback_image \
    shader_prepass \
    extended_material \
    shader_material_wesl \
    shader_material_bindless \
    extended_material_bindless

# ssr: OK
# lines: OK
# ui_material: OK
# tonemapping: no wesl
# shader_defs: OK
# clustered_decals: OK
# gpu_readback: ??
# array_texture: OK
# storage_buffer: OK
# irradiance_volumes: OK
# animate_shader: OK
# fallback_image: looks broken, likely caused by strip (frag empty)
# shader_prepass: wesl: error: module `bevy::pbr::mesh_view_bindings` has no declaration `depth_prepass_texture`
# shader_material: OK
# extended_material: OK
# custom_phase_item: OK
# shader_material_2d: OK
# custom_render_phase: OK
# automatic_instancing: OK
# shader_material_wesl: 
#  In Device::create_render_pipeline, label = 'opaque_mesh_pipeline'
#    Error matching ShaderStages(FRAGMENT) shader requirements against the pipeline
#      Shader global ResourceBinding { group: 2, binding: 1 } is not available in the pipeline layout
#        Binding is missing from the pipeline layout
# texture_binding_array: OK
# custom_post_processing: OK
# custom_vertex_attribute: OK
# custom_gltf_vertex_attribute: OK
# custom_shader_instancing: OK
# shader_material_bindless: 
#  In Device::create_render_pipeline, label = 'opaque_mesh_pipeline'
#    Error matching ShaderStages(FRAGMENT) shader requirements against the pipeline
#      Shader global ResourceBinding { group: 2, binding: 0 } is not available in the pipeline layout
#        Buffer structure size 16, added to one element of an unbound array, if it's the last field, ended up greater than the given `min_binding_size`, which is 12
# specialized_mesh_pipeline: OK
# extended_material_bindless: path not found
# shader_material_screenspace_texture: OK

for example in $examples
    echo running $example
    WESL_LOWER=1 WESL_STRIP=1 WESL_VALIDATE=1 cargo run --features shader_format_wesl --example $example
end
