#version 140
uniform sampler2D tex;
out vec4 f_color;
in vec2 v_tex_coord;

void main() {
	f_color = texture(tex, v_tex_coord);
}