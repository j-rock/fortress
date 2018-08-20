#version 330 core

layout (location = 0) out vec3 position;
layout (location = 1) out vec3 normal;
layout (location = 2) out vec4 color; // rgb = diffuse, a = specular

in GS_OUT {
  vec3 world_space_position;
  vec4 color;
} gs_in;

void main() {
	position = gs_in.world_space_position;
	color = gs_in.color;
	normal = vec3(0.0, 1.0, 0.0);
}