#version 330 core

struct PlatformAttr {
    vec2 bottom_left;
    vec2 top_right;
};

layout (location = 0) in PlatformAttr platform;

out PlatformAttr vs_out;

void main() {
	vs_out = platform;
    gl_Position = vec4(platform.bottom_left, 0.0, 1.0);
}
