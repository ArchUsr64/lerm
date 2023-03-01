#version 450

in vec2 pos;
in vec2 uv;

out vec2 frag_uv;

void main() {
	frag_uv = uv;
	gl_Position = vec4(pos, 0., 1.);
}
