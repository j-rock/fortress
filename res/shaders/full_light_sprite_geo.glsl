#version 330 core
layout (points) in;
layout (triangle_strip, max_vertices = 4) out;

in VS_OUT {
    vec3 world_center_position;
    vec2 half_size;
    vec2 texel_bottom_left;
    vec2 texel_top_right;
    float rotation;
} gs_in[];

out GS_OUT {
    vec3 world_space_position;
    vec3 normal;
    vec2 texel;
} gs_out;

uniform mat4 projection_view;
uniform vec3 camera_right;
uniform vec3 camera_up;

mat3 RotationMatrix(vec3 axis, float angle) {
    float deg_to_rad = 0.0174533;
    float cos_rot = cos(gs_in[0].rotation * deg_to_rad);
    float sin_rot = sin(gs_in[0].rotation * deg_to_rad);
    float o_m_cos = 1 - cos_rot;
    float x_om_cos = axis.x * o_m_cos;
    float y_om_cos = axis.y * o_m_cos;
    float x_sin = axis.x * sin_rot;
    float y_sin = axis.y * sin_rot;
    float z_sin = axis.z * sin_rot;

    return mat3(cos_rot + axis.x * x_om_cos,   axis.y * x_om_cos - z_sin,           axis.z * x_om_cos + y_sin,
                  axis.y * x_om_cos + z_sin, cos_rot + axis.y * y_om_cos,           axis.z * y_om_cos - x_sin,
                  axis.z * x_om_cos - y_sin,   axis.z * y_om_cos + x_sin, cos_rot + axis.z * axis.z * o_m_cos);
}

void EmitQuad() {
    gs_out.normal = cross(camera_right, camera_up);

    vec3 world_bottom_center = gs_in[0].world_center_position;

    mat3 rot = RotationMatrix(gs_out.normal, gs_in[0].rotation);
    vec3 rot_camera_right = rot * camera_right;
    vec3 rot_camera_up = rot * camera_up;

    vec3 half_size_along_camera_right = gs_in[0].half_size.x * rot_camera_right;
    vec3 half_size_along_camera_up = gs_in[0].half_size.y * rot_camera_up;

    vec3 world_bottom_left = world_bottom_center - half_size_along_camera_right - half_size_along_camera_up;
    vec3 world_bottom_right = world_bottom_center + half_size_along_camera_right - half_size_along_camera_up;
    vec3 world_top_left = world_bottom_center - half_size_along_camera_right + half_size_along_camera_up;
    vec3 world_top_right = world_bottom_center + half_size_along_camera_right + half_size_along_camera_up;

    vec2 texel_top_left = vec2(gs_in[0].texel_bottom_left.x, gs_in[0].texel_top_right.y);
    vec2 texel_bottom_right = vec2(gs_in[0].texel_top_right.x, gs_in[0].texel_bottom_left.y);

    gl_Position = projection_view * vec4(world_top_left, 1.0);
    gs_out.world_space_position = world_top_left;
    gs_out.texel = texel_top_left;
    EmitVertex();

    gl_Position = projection_view * vec4(world_bottom_left, 1.0);
    gs_out.world_space_position = world_bottom_left;
    gs_out.texel = gs_in[0].texel_bottom_left;
    EmitVertex();

    gl_Position = projection_view * vec4(world_top_right, 1.0);
    gs_out.world_space_position = world_top_right;
    gs_out.texel = gs_in[0].texel_top_right;
    EmitVertex();

    gl_Position = projection_view * vec4(world_bottom_right, 1.0);
    gs_out.world_space_position = world_bottom_right;
    gs_out.texel = texel_bottom_right;
    EmitVertex();

    EndPrimitive();
}

void main() {
    EmitQuad();
}