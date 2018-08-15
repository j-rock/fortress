#version 330 core

layout (location = 0) out vec3 position;
layout (location = 1) out vec3 normal;
layout (location = 2) out vec4 color; // rgb = diffuse, a = specular

in GS_OUT {
  vec3 world_space_position;
  vec3 color;
} gs_in;

void main() {
	position = gs_in.world_space_position;
	color.rgb = gs_in.color;
	color.a = 0.0;
	normal = vec3(0.0, 1.0, 0.0);
}