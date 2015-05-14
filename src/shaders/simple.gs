#version 330

uniform mat4 persp_matrix;
uniform mat4 view_matrix;

layout(triangles) in;
layout(triangle_strip, max_vertices=3) out;

out VertexData {
    vec3 color;
} v_out;


float rand(vec2 co) {
    return fract(sin(dot(co.xy ,vec2(12.9898,78.233))) * 43758.5453);
}

void main() {
    mat4 matrix = persp_matrix * view_matrix;

    vec3 all_color = vec3(
        rand(gl_in[0].gl_Position.xy + gl_in[1].gl_Position.yz),
        rand(gl_in[1].gl_Position.yx + gl_in[2].gl_Position.zx),
        rand(gl_in[2].gl_Position.xz + gl_in[2].gl_Position.zy)
    );

    gl_Position = gl_in[0].gl_Position;
    //v_out.tex_coords = v_in[0].tex_coords;
    v_out.color = all_color;
    EmitVertex();

    gl_Position = gl_in[1].gl_Position;
    //v_out.tex_coords = v_in[1].tex_coords;
    v_out.color = all_color;
    EmitVertex();

    gl_Position = gl_in[2].gl_Position;
    //v_out.tex_coords = v_in[2].tex_coords;
    v_out.color = all_color;
    EmitVertex();
}
