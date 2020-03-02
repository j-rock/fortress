#version 330 core
layout (points) in;
layout (triangle_strip, max_vertices = 4) out;

in VS_OUT {
    vec3 position;
    vec3 color;
    float alpha;
    float size;
} gs_in[];

out GS_OUT {
    vec3 color;
    float alpha;
} gs_out;

uniform mat4 projection_view;
uniform vec3 camera_right;
uniform vec3 camera_up;

void EmitV(vec3 world_position) {
    gl_Position = projection_view * vec4(world_position, 1.0);
    EmitVertex();
}

void main() {
    gs_out.color = gs_in[0].color;
    gs_out.alpha = gs_in[0].alpha;

    float half_size = gs_in[0].size / 2.0;
    vec3 half_right = half_size * camera_right;
    vec3 half_up = half_size * camera_up;

    vec3 top_left = gs_in[0].position - half_right + half_up;
    vec3 top_right = gs_in[0].position + half_right + half_up;
    vec3 bot_left = gs_in[0].position - half_right - half_up;
    vec3 bot_right = gs_in[0].position + half_right - half_up;

    EmitV(top_left);
    EmitV(top_right);
    EmitV(bot_left);
    EmitV(bot_right);
    EndPrimitive();
}
