#version 410

uniform sampler2D grass_texture_unit;

in VertexData {
    vec2 tex_coord;
} v_in;

out vec4 output1;



void main() {
    output1 = texture(grass_texture_unit, v_in.tex_coord);
}
