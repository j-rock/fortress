#version 330 core
layout (location = 0) in vec4 platform_vec4;

void main() {
    gl_Position = platform_vec4;
}
