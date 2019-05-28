#version 330 core
layout (location = 0) in vec3 position;
layout (location = 1) in vec3 color;

out VS_OUT {
    vec3 color;
} vs_out;

uniform mat4 projection_view;
uniform float particle_size;

void main()
{
    gl_Position = projection_view * vec4(position, 1.0);
    gl_PointSize = particle_size;
    vs_out.color = color;
}
