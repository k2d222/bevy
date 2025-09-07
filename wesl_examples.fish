set examples \
    ssr \
    lines \
    ui_material \
    tonemapping \
    shader_defs \
    clustered_decals \
    gpu_readback \
    array_texture \
    storage_buffer \
    irradiance_volumes \
    animate_shader \
    fallback_image \
    shader_prepass \
    shader_material \
    extended_material \
    custom_phase_item \
    shader_material_2d \
    custom_render_phase \
    automatic_instancing \
    shader_material_wesl \
    texture_binding_array \
    custom_post_processing \
    custom_vertex_attribute \
    custom_gltf_vertex_attribute \
    custom_shader_instancing \
    shader_material_bindless \
    specialized_mesh_pipeline \
    extended_material_bindless \
    shader_material_screenspace_texture

# set examples \
#     fallback_image \
#     shader_material_wesl \
#     shader_material_bindless

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
# fallback_image: looks broken, fragment shader is empty
# shader_prepass: OK
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

# EXAMPLE: TIME (with eval), TIME (no eval)
# ssr: 466, 95
# lines: 10, 2
# ui_material: 13, 4
# tonemapping: N/A, N/A
# shader_defs: 11 + 4, 2 + 2
# clustered_decals: 679, 114
# gpu_readback: N/A, N/A
# array_texture: 559, 95
# storage_buffer: 118, 29
# irradiance_volumes: 126, 34
# animate_shader: 35, 13
# fallback_image: 19, 3
# shader_prepass: 11 + 32 + 4, 13 + 3 + 12
# shader_material: 12, 3
# extended_material: 638, 
# custom_phase_item: 11, 
# shader_material_2d: 10, 
# custom_render_phase: 130, 
# automatic_instancing: 121, 
# shader_material_wesl: 11, 
# texture_binding_array: 14, 
# custom_post_processing: 13, 
# custom_vertex_attribute: 108, 
# custom_gltf_vertex_attribute: 50, 
# custom_shader_instancing: 114, 
# shader_material_bindless: 14, 
# specialized_mesh_pipeline: 104, 
# extended_material_bindless: 654, 
# shader_material_screenspace_texture: 47, 

for example in $examples
    echo running $example
    WESL_LOWER=1 WESL_STRIP=1 WESL_VALIDATE=1 cargo run --features shader_format_wesl --example $example
end
