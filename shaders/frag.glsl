#version 450
in vec2 frag_uv;
out vec4 frag_color;

uniform sampler2D texture_atlas;

void main() {
	frag_color = 1. - vec4(vec3(texture(texture_atlas, frag_uv).a), 1.);
}
