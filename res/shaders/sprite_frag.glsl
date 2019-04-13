#version 330 core

layout (location = 0) out vec3 position;
layout (location = 1) out vec3 normal;
layout (location = 2) out vec4 color; // rgb = diffuse, a = specular

in GS_OUT {
	vec3 world_space_position;
    vec3 normal;
	vec2 texel;
} fs_in;

uniform sampler2D texture0;

void main() {
	position = fs_in.world_space_position;
	normal = fs_in.normal;
	color = texture(texture0, fs_in.texel);
}