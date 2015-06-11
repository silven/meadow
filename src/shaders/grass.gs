#version 330

layout(points) in;
layout(triangle_strip, max_vertices = 32) out;

uniform vec3 windforce;
uniform mat4 persp_matrix;
uniform mat4 view_matrix;

in VertexData {
    float rand_factor;
} v_in[1];

out VertexData {
    vec2 tex_coord;
    float rand_factor;
} v_out;


float rand(vec2 co) {
    return fract(sin(dot(co.xy, vec2(12.9898, 78.233))) * 43758.5453);
}

void main(){
    mat4 totalMatrix = persp_matrix * view_matrix;

    vec4 seedPos = gl_in[0].gl_Position;
    float personal = rand(seedPos.xz);
    float personal2 = v_in[0].rand_factor;
    seedPos.x += personal - 0.5;
    seedPos.z += v_in[0].rand_factor - 0.5;

    vec4 up = vec4(fract(personal / 10), 1.0, -fract(personal2 / 10), 0.0);

    float height = 1.0;
    float width = 0.02;
    vec4 right = vec4(normalize(vec3(personal, 0.0, v_in[0].rand_factor)), 0.0);
    vec4 left = vec4(cross(up.xyz, right.xyz), 0);

    vec4 windVec = vec4(0.8 * windforce, 0.0);
    vec4 Ydisplacement = 0.5 * length(windVec) * up;

    // Side one
    int lod = 5;
    for(int i = 0; i <= lod; i++) {
        float progress = (i/float(lod));
        float next = (i+1/float(lod));

        vec4 windOffset = (progress * progress) * windVec;
        vec4 heightOffset = progress * up * height - (progress * progress) * Ydisplacement;;

        // Middle point
        vec4 firstSide = (seedPos + heightOffset + windOffset);

        v_out.tex_coord = vec2(0.5, progress);
        v_out.rand_factor = v_in[0].rand_factor;
        gl_Position = totalMatrix * firstSide;
        EmitVertex();

        // Right point (first side + width)
        gl_Position = totalMatrix * (firstSide + right * width);
        v_out.rand_factor = v_in[0].rand_factor;
        v_out.tex_coord = vec2(1.0, progress);
        EmitVertex();
    }
    EndPrimitive();

    for(int j = 0; j <= lod; j++) {
        float progress = (j / float(lod));
        float next = (j+1 / float(lod));

        vec4 windOffset = (progress * progress) * windVec;
        vec4 heightOffset = progress * up * height - (progress * progress) * Ydisplacement;;

        // Middle point
        vec4 firstSide = (seedPos + heightOffset + windOffset);

        v_out.tex_coord = vec2(0.5, progress);
        v_out.rand_factor = v_in[0].rand_factor;
        gl_Position = totalMatrix * firstSide;
        EmitVertex();

        // Right point (first side - width)
        gl_Position = totalMatrix * (firstSide - left * width);
        v_out.tex_coord = vec2(0.0, progress);
        v_out.rand_factor = v_in[0].rand_factor;
        EmitVertex();
    }
    EndPrimitive();
}

