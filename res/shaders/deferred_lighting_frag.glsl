#version 330 core

out vec4 frag_color;
  
in vec2 texture_coords;

uniform sampler2D position_tex;
uniform sampler2D normal_tex;
uniform sampler2D color_tex;

void main()
{
    vec3 position = texture(position_tex, texture_coords).rgb;
    vec3 normal = texture(normal_tex, texture_coords).rgb;
	vec4 diffuse_specular = texture(color_tex, texture_coords);
	vec3 diffuse_color = diffuse_specular.rgb;
	float specular_color = diffuse_specular.a;

	vec3 light_color = vec3(1.0);
	vec3 light_position = vec3(32.5, 9, 0.0);
	float light_constant = 1.0;
	float light_linear = 0.005;
	float light_quadratic = 0.0004;

	// Ambient
	float amb = 0.9f;
	vec3 ambient_light = amb * light_color;

	// Diffuse
	// vec3 light_dir = normalize(-vec3(0.0, 0.0, 1.0));
	vec3 light_dir = normalize(light_position - position);
	float diffuse_intensity = max(dot(normal, light_dir), 0.0);
	vec3 diffuse_light = diffuse_intensity * light_color;

	// Point light attenuation
	float distance = length(light_position - position);
	float attenuation = 1.0 / (light_constant + light_linear * distance + light_quadratic * (distance * distance)); 

	// Merge
	vec3 color = (ambient_light + diffuse_light) * diffuse_color * attenuation;

	// Gamma correct
	color = pow(color, vec3(1.0/2.2));
    frag_color = vec4(color, 1.0);
}