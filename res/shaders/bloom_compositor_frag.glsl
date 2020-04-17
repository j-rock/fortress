#version 330 core

out vec4 frag_color;

in VS_OUT {
    vec2 texel;
} fs_in;

uniform sampler2D scene;
uniform sampler2D bloom;
uniform float bloom_intensity_multiplier;

void main() {
    vec4 scene_color = texture(scene, fs_in.texel);
    vec4 bloom_color = texture(bloom, fs_in.texel);
    float bloom_intensity = bloom_intensity_multiplier * bloom_color.a;
    vec3 render_color = scene_color.rgb + bloom_intensity * bloom_color.rgb;
    frag_color = vec4(render_color, scene_color.a);
}