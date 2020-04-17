#version 330 core
layout (location = 0) out vec4 frag_color;
layout (location = 1) out vec4 bloom_color;

in GS_OUT {
	vec3 world_space_position;
	vec2 texel;
	float bloom_intensity;
} fs_in;

uniform sampler2D texture0;

void main() {
	vec3 position = fs_in.world_space_position;
	vec4 color = texture(texture0, fs_in.texel);
	vec3 diffuse_color = color.rgb;

    if (color.a < 0.01) {
        discard;
    }

	// Gamma correct
	vec3 lighting = pow(diffuse_color, vec3(1.0/2.2));
	frag_color = vec4(lighting, 1.0);
	bloom_color = vec4(frag_color.rgb, fs_in.bloom_intensity);
}