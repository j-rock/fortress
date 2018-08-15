#version 330 core
layout (location = 0) in vec4 wraith_vec4;

void main() {
    gl_Position = wraith_vec4;
}
