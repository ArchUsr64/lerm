#version 420
in vec4 vert_out_color;
out vec4 frag_color;

void main() {
	frag_color = vert_out_color;
}
