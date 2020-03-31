#version 330 core
layout (location = 0) out vec4 frag_color;
layout (location = 1) out vec4 bloom_color;

in vec2 texture_coords;

uniform sampler2D texture0;

void main() {
    vec4 srgb_alpha = texture(texture0, texture_coords);
    // Gamma correct.
    vec3 rgb = pow(srgb_alpha.rgb, vec3(1.0/2.2));
    frag_color = vec4(rgb, srgb_alpha.a);
    bloom_color = vec4(0.0, 0.0, 0.0, 1.0);
}