#version 410

uniform sampler2D texture_unit;

in VertexData {
    vec2 tex_coords;
} v_in;

out vec4 output1;

void main() {
    output1 = texture(texture_unit, v_in.tex_coords);
    //output1 = vec4(1.0, 0.0, 1.0, 1.0);
}
