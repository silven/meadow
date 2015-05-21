#version 330

layout(points) in;
layout(triangle_strip, max_vertices = 32) out;

uniform vec3 windforce;
uniform mat4 persp_matrix;
uniform mat4 view_matrix;

out VertexData {
    vec2 tex_coord;
} v_out;


void main(){
    mat4 totalMatrix = persp_matrix * view_matrix;

    vec4 seedPos = gl_in[0].gl_Position;
    vec4 up = vec4(0.0, 1.0, 0.0, 0.0);

    float height = 2.0;
    float width = 0.1;
    vec4 right = vec4(1.0, 0.0, 0.0, 0.0);
    vec4 left = vec4(cross(up.xyz, right.xyz), 0);


    vec4 windVec = vec4(0.8 * windforce, 0.0);
    vec4 Ydisplacement = 0.5 * length(windVec) * up;

    // Side one
    int lod = 3;
    for(int i = 0; i <= lod; i++) {
        float progress = (i/float(lod));
        float next = (i+1/float(lod));

        vec4 windOffset = (progress * progress) * windVec;
        vec4 heightOffset = progress * up * height - (progress * progress) * Ydisplacement;;

        // Middle point
        vec4 firstSide = (seedPos + heightOffset + windOffset);

        v_out.tex_coord = vec2(0.5, progress);
        gl_Position = totalMatrix * firstSide;
        EmitVertex();

        // Right point (first side + width)
        gl_Position = totalMatrix * (firstSide + right * width);
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
        gl_Position = totalMatrix * firstSide;
        EmitVertex();

        // Right point (first side - width)
        gl_Position = totalMatrix * (firstSide - left * width);
        v_out.tex_coord = vec2(0.0, progress);
        EmitVertex();
    }
    EndPrimitive();
}

