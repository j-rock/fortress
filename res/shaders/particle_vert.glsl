#version 330 core
layout (location = 0) in vec3 position;
layout (location = 1) in vec3 color;
layout (location = 2) in float alpha;

out VS_OUT {
    vec3 position;
    vec3 color;
    float alpha;
} vs_out;

void main()
{
    gl_Position = vec4(position, 1.0);
    vs_out.position = position;
    vs_out.color = color;
    vs_out.alpha = alpha;
}
