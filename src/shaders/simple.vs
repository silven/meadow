#version 410

uniform mat4 persp_matrix;
uniform mat4 view_matrix;

in vec3 position;
in vec2 tex_coords;

out VertexData {
    vec2 tex_coords;
} v_out;

void main() {
    v_out.tex_coords = tex_coords;
    gl_Position = persp_matrix * view_matrix * vec4(position, 1.0);
}
