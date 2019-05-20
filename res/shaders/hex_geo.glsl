#version 330 core
layout (triangles) in;
layout (triangle_strip, max_vertices = 3) out;

in VS_OUT {
    vec3 world_space_position;
} gs_in[];

out GS_OUT {
    vec3 world_space_position;
    vec3 normal;
} gs_out;

vec3 GetNormal() {
    vec3 a = gs_in[1].world_space_position - gs_in[0].world_space_position;
    vec3 b = gs_in[2].world_space_position - gs_in[0].world_space_position;
    return normalize(cross(a, b));
}

void EmitV(int index) {
    gl_Position = gl_in[index].gl_Position;
    gs_out.world_space_position = gs_in[index].world_space_position;
    EmitVertex();
}

void main() {
    gs_out.normal = GetNormal();
    EmitV(0);
    EmitV(1);
    EmitV(2);
    EndPrimitive();
}
