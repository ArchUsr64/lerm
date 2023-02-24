#version 450

in vec2 pos;
in vec3 color;

out vec4 vert_out_color;

void main() {
	vert_out_color = vec4(color, 1.);
	gl_Position = vec4(pos, 1., 1.);
}
