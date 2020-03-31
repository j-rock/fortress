#version 330 core

out vec4 frag_color;

in VS_OUT {
    vec2 texel;
} fs_in;

uniform sampler2D scene;
uniform sampler2D bloom;

void main() {
    vec4 scene_color = texture(scene, fs_in.texel);
    vec4 bloom_color = texture(bloom, fs_in.texel);

    if (scene_color == bloom_color) {
        frag_color = vec4(0.0, 0.0, 0.0, 1.0);
    } else {
        frag_color = vec4(scene_color.rgb, bloom_color.a);
    }
}