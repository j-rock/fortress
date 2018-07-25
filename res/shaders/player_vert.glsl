#version 330 core
layout (location = 0) in vec4 player_vec4;

void main() {
    gl_Position = player_vec4;
}
