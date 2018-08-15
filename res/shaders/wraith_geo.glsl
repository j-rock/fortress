#version 330 core

layout (points) in;
layout (triangle_strip, max_vertices = 4) out;

struct WraithAttr {
    vec2 position;
    vec2 half_size;
};

out GS_OUT {
  vec3 world_space_position;
  vec3 color;
} gs_out;

uniform mat4 projection_view;

void EmitQuad(in WraithAttr wraith) {
    float left = wraith.position.x - wraith.half_size.x;
    float right = wraith.position.x + wraith.half_size.x;
    float top = wraith.position.y + wraith.half_size.y;
    float bottom = wraith.position.y - wraith.half_size.y;

    vec4 v4;


	v4 = vec4(left, top, 0.0, 1.0);
	gl_Position = projection_view * v4;
	gs_out.world_space_position = v4.xyz;
	gs_out.color = vec3(0.8, 0.1, 0.1);
	EmitVertex();

	v4 = vec4(left, bottom, 0.0, 1.0);
	gl_Position = projection_view * v4;
	gs_out.world_space_position = v4.xyz;
	gs_out.color = vec3(1.0, 0.0, 0.1);
	EmitVertex();

	v4 = vec4(right, top, 0.0, 1.0);
	gl_Position = projection_view * v4;
	gs_out.world_space_position = v4.xyz;
	gs_out.color = vec3(1.0, 0.1, 0.1);
	EmitVertex();

	v4 = vec4(right, bottom, 0.0, 1.0);
	gl_Position = projection_view * v4;
	gs_out.world_space_position = v4.xyz;
	gs_out.color = vec3(0.5, 0.0, 0.0);
	EmitVertex();

	EndPrimitive();
}

void main() {
  WraithAttr wraith;
  wraith.position = gl_in[0].gl_Position.xy;
  wraith.half_size = gl_in[0].gl_Position.zw;
  EmitQuad(wraith);
}
