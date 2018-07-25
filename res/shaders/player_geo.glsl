#version 330 core

layout (points) in;
layout (triangle_strip, max_vertices = 4) out;

struct PlayerAttr {
    vec2 position;
    vec2 half_size;
};

out GS_OUT {
  vec3 world_space_position;
  vec3 color;
} gs_out;

uniform mat4 projection_view;

void EmitQuad(in PlayerAttr player) {
    float left = player.position.x - player.half_size.x;
    float right = player.position.x + player.half_size.x;
    float top = player.position.y + player.half_size.y;
    float bottom = player.position.y - player.half_size.y;

    vec4 v4;


	v4 = vec4(left, top, 0.0, 1.0);
	gl_Position = projection_view * v4;
	gs_out.world_space_position = v4.xyz;
	gs_out.color = vec3(0.8, 0.3, 0.3);
	EmitVertex();

	v4 = vec4(left, bottom, 0.0, 1.0);
	gl_Position = projection_view * v4;
	gs_out.world_space_position = v4.xyz;
	gs_out.color = vec3(0.4, 0.0, 0.8);
	EmitVertex();

	v4 = vec4(right, top, 0.0, 1.0);
	gl_Position = projection_view * v4;
	gs_out.world_space_position = v4.xyz;
	gs_out.color = vec3(0.2, 0.2, 1.0);
	EmitVertex();

	v4 = vec4(right, bottom, 0.0, 1.0);
	gl_Position = projection_view * v4;
	gs_out.world_space_position = v4.xyz;
	gs_out.color = vec3(1.0, 0.0, 0.1);
	EmitVertex();

	EndPrimitive();
}

void main() {
  PlayerAttr player;
  player.position = gl_in[0].gl_Position.xy;
  player.half_size = gl_in[0].gl_Position.zw;
  EmitQuad(player);
}
