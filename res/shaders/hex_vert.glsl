#version 330 core
layout (location = 0) in vec3 local_vertex_position;
layout (location = 1) in vec4 transform; // x,y = 2D coords, z = height, w = elevation.
layout (location = 2) in float hexagon_scale;
layout (location = 3) in float hexagon_alpha;

out VS_OUT {
    vec3 world_space_position;
    float hexagon_alpha;
} vs_out;

uniform mat4 projection_view;
uniform float bevel_raise;

void main() {
    vec2 translation = transform.xy;
    float height = transform.z;
    float elevation = transform.w;

    vec2 xz_pos = hexagon_scale * local_vertex_position.xz + translation;
    float y_pos = hexagon_scale * height * (local_vertex_position.y - bevel_raise) + elevation;
    vec3 world_space_position = vec3(xz_pos.x, y_pos, xz_pos.y);

    gl_Position = projection_view * vec4(world_space_position, 1.0);
    vs_out.world_space_position = world_space_position;
    vs_out.hexagon_alpha = hexagon_alpha;
}
