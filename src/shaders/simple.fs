#version 410

uniform sampler2D texture_unit;

in VertexData {
    vec2 tex_coords;
} v_in;

out vec4 output1;

void main() {
    vec3 texture_color = texture(texture_unit, v_in.tex_coords).xyz;
    output1 = vec4(texture_color, 1.0);
}
