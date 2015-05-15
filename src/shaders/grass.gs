#version 330

layout(points) in;
layout(triangle_strip, max_vertices = 32) out;

uniform mat4 persp_matrix;
uniform mat4 view_matrix;

void main(){

    mat4 totalMatrix = persp_matrix * view_matrix;

    vec4 seedPos = gl_in[0].gl_Position;
    vec4 heightVec = vec4(0.0, 2.0, 0.0, 0.0);

    // Side one
    int lod = 3;
    for(int i = 0; i <= lod; i++) {
        float progress = (i/float(lod));
        float next = (i+1/float(lod));

        vec4 heightOffset = progress * heightVec;

        // Middle point
        vec4 firstSide = (seedPos + heightOffset);

        gl_Position = totalMatrix * firstSide;
        EmitVertex();

        // Right point (first side + width)
        gl_Position = totalMatrix * (firstSide + vec4(1.0, 0.0, 0.0, 0.0));
        EmitVertex();
    }
    EndPrimitive();
}

