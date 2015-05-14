#version 410

uniform sampler2D texture_unit;

in VertexData {
    vec2 tex_coords;
} v_in;

out vec4 o_Color;

void main() {
    o_Color = texture(texture_unit, v_in.tex_coords);
}
