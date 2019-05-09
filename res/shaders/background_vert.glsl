#version 330 core
layout (location = 0) in vec2 position;
layout (location = 1) in vec2 in_texture_coords;

out vec2 texture_coords;

void main()
{
    texture_coords = vec2(in_texture_coords.x, 1.0 - in_texture_coords.y);
    gl_Position = vec4(position.x, position.y, 0.0, 1.0);
}