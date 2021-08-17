#version 140
uniform mat4 proj_matrix;
uniform mat4 view_matrix;
in vec2 position;
in vec2 tex_coord;

out vec2 v_tex_coord;
void main() {
	gl_Position = proj_matrix * view_matrix * vec4(position, 0.0, 1.0);
	v_tex_coord = tex_coord;
}