#version 410

uniform sampler2D grass_texture_unit;
uniform sampler2D mask_texture_unit;

in VertexData {
    vec2 tex_coord;
} v_in;

out vec4 output1;



void main() {

    vec4 grass_color = texture(grass_texture_unit, v_in.tex_coord);

    float blend = 255 * texture(mask_texture_unit, v_in.tex_coord).x;
    float alpha = (1 - blend) * (grass_color.a - 0.5) + 0.5;

    vec3 darker_color = v_in.tex_coord.y * grass_color.xyz;

    if(alpha < 0.3) discard;
    output1 = vec4(darker_color, alpha);
}
