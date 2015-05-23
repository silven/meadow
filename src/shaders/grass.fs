#version 410

uniform sampler2D grass_texture_unit;
uniform sampler2D mask_texture_unit;

in VertexData {
    vec2 tex_coord;
    float rand_factor;
} v_in;

out vec4 output1;


void main() {
    vec4 grass_color = texture(grass_texture_unit, v_in.tex_coord);

    float blend = 255 * texture(mask_texture_unit, v_in.tex_coord).x;
    float alpha = (1 - blend) * (grass_color.a - 0.5) + 0.5;

    vec3 orange_tint = 0.5 * v_in.rand_factor * vec3(0.8, 0.5, 0.0);
    vec3 darker_color = clamp(v_in.tex_coord.y, 0.1, 1.0) * grass_color.xyz;

    if(alpha < 0.3) discard;
    output1 = vec4(mix(darker_color, orange_tint, v_in.rand_factor / 2.0), alpha);
}
