#version 330 core

out vec4 frag_color;

in vec2 texture_coords;

uniform sampler2D texture0;

void main() {
    frag_color = texture(texture0, texture_coords);
}