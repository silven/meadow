#version 410

in vec3 position;
in vec2 tex_coords;

out VertexData {
    vec2 tex_coords;
} v_out;

void main() {
    v_out.tex_coords = tex_coords;
    gl_Position = vec4(position, 1.0);
}
