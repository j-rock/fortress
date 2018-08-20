#version 330 core

layout (points) in;
layout (triangle_strip, max_vertices = 4) out;

in VS_OUT {
  vec2 position;
  vec2 half_size;
  vec4 color_tl;
  vec4 color_tr;
  vec4 color_bl;
  vec4 color_br;
} gs_in[];

out GS_OUT {
  vec3 world_space_position;
  vec4 color;
} gs_out;

uniform mat4 projection_view;

void EmitQuad() {
    float left = gs_in[0].position.x - gs_in[0].half_size.x;
    float right = gs_in[0].position.x + gs_in[0].half_size.x;
    float top = gs_in[0].position.y + gs_in[0].half_size.y;
    float bottom = gs_in[0].position.y - gs_in[0].half_size.y;

    vec4 v4 = vec4(left, top, 0.0, 1.0);
	gl_Position = projection_view * v4;
	gs_out.world_space_position = v4.xyz;
	gs_out.color = gs_in[0].color_tl;
	EmitVertex();

	v4 = vec4(left, bottom, 0.0, 1.0);
	gl_Position = projection_view * v4;
	gs_out.world_space_position = v4.xyz;
	gs_out.color = gs_in[0].color_bl;
	EmitVertex();

	v4 = vec4(right, top, 0.0, 1.0);
	gl_Position = projection_view * v4;
	gs_out.world_space_position = v4.xyz;
	gs_out.color = gs_in[0].color_tr;
	EmitVertex();

	v4 = vec4(right, bottom, 0.0, 1.0);
	gl_Position = projection_view * v4;
	gs_out.world_space_position = v4.xyz;
	gs_out.color = gs_in[0].color_br;
	EmitVertex();

	EndPrimitive();
}

void main() {
  EmitQuad();
}
