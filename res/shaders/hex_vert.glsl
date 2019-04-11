#version 330 core
layout (location = 0) in vec3 local_vertex_position;
layout (location = 1) in vec3 mesh_position;
layout (location = 2) in float height;
layout (location = 3) in vec4 rgba_color;

out VS_OUT {
    vec3 world_space_position;
    vec4 rgba_color;
} vs_out;

uniform mat4 projection_view;

void main() {
    vec3 local_scaled_for_height = vec3(local_vertex_position.x, local_vertex_position.y * height, local_vertex_position.z);
    vec3 world_space_position = local_scaled_for_height + mesh_position;

    gl_Position = projection_view * vec4(world_space_position, 1.0);
    vs_out.world_space_position = world_space_position;
    vs_out.rgba_color = rgba_color;
}
