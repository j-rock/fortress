#version 330 core

out vec4 frag_color;

in GS_OUT {
	vec3 world_space_position;
    vec3 normal;
	vec2 texel;
} fs_in;

uniform sampler2D texture0;

void main() {
	vec3 position = fs_in.world_space_position;
	vec3 normal = fs_in.normal;
	vec4 color = texture(texture0, fs_in.texel);
	vec3 diffuse_color = color.rgb;

    if (color.a < 0.01) {
        discard;
    }

	// Gamma correct
	vec3 lighting = pow(diffuse_color, vec3(1.0/2.2));
	frag_color = vec4(lighting, 1.0);
}