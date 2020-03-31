#version 330 core

out vec4 frag_color;

in VS_OUT {
    vec2 texel;
} fs_in;

uniform sampler2D image;
uniform bool horizontal;

const float kernel[5] = float[] (0.2270270270, 0.1945945946, 0.1216216216, 0.0540540541, 0.0162162162);

void main() {
    vec2 texel_size = 1.0 / textureSize(image, 0);
    vec4 image_color = texture(image, fs_in.texel);
    vec3 result = image_color.rgb * kernel[0];

    if (horizontal) {
        for (int i = 1; i < 5; ++i) {
            vec2 offset = vec2(texel_size.x * i, 0.0);
            result += texture(image, fs_in.texel + offset).rgb * kernel[i];
            result += texture(image, fs_in.texel - offset).rgb * kernel[i];
        }
    } else {
        for (int i = 1; i < 5; ++i) {
            vec2 offset = vec2(0.0, texel_size.y * i);
            result += texture(image, fs_in.texel + offset).rgb * kernel[i];
            result += texture(image, fs_in.texel - offset).rgb * kernel[i];
        }
    }

    frag_color = vec4(result, 1.0);
}