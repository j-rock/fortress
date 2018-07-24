#version 330 core

layout (points) in;
layout (triangle_strip, max_vertices = 4) out;

struct PlatformAttr {
    vec2 bottom_left;
    vec2 top_right;
};

in PlatformAttr vs_in[];

out GS_OUT {
  vec3 world_space_position;
  vec3 color;
} gs_out;

uniform mat4 projection_view;

void EmitQuad(in PlatformAttr platform) {
    vec4 v4;

	v4 = vec4(platform.bottom_left.x, platform.top_right.y, 0.0, 1.0);
	gl_Position = projection_view * v4;
	gs_out.world_space_position = v4.xyz;
	gs_out.color = vec3(0.3, 0.0, 4.0);
	EmitVertex();

	v4 = vec4(platform.bottom_left, 0.0, 1.0);
	gl_Position = projection_view * v4;
	gs_out.world_space_position = v4.xyz;
	gs_out.color = vec3(0.0, 0.8, 0.0);
	EmitVertex();

	v4 = vec4(platform.top_right, 0.0, 1.0);
	gl_Position = projection_view * v4;
	gs_out.world_space_position = v4.xyz;
	gs_out.color = vec3(0.5, 0.5, 1.0);
	EmitVertex();

	v4 = vec4(platform.top_right.x, platform.bottom_left.y, 0.0, 1.0);
	gl_Position = projection_view * v4;
	gs_out.world_space_position = v4.xyz;
	gs_out.color = vec3(1.0, 0.5, 0.0);
	EmitVertex();

	EndPrimitive();
}

void main() {
  EmitQuad(vs_in[0]);
}
