#version 330 core
layout (points) in;
layout (triangle_strip, max_vertices = 4) out;

in VS_OUT {
  vec3 world_bottom_center_position;
  vec2 half_size;
  vec2 texel_bottom_left;
  vec2 texel_top_right;
} gs_in[];

out GS_OUT {
  vec3 world_space_position;
  vec2 texel;
} gs_out;

uniform mat4 projection_view;
uniform vec3 camera_right;
uniform vec3 camera_up;

void EmitQuad() {
  vec3 half_size_along_camera_right = gs_in[0].half_size.x * camera_right;
  vec3 half_size_along_camera_up = gs_in[0].half_size.y * camera_up;

  vec3 world_bottom_left = gs_in[0].world_bottom_center_position - half_size_along_camera_right;
  vec3 world_bottom_right = gs_in[0].world_bottom_center_position + half_size_along_camera_right;
  vec3 world_top_left = world_bottom_left + 2.0 * half_size_along_camera_up;
  vec3 world_top_right = world_top_left + 2.0 * half_size_along_camera_right;

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
