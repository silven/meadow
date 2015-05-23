#version 410

in vec3 position;
in vec3 offset;
in float rand_factor;

out VertexData {
    float rand_factor;
} v;

void main() {
    v.rand_factor = rand_factor;
    gl_Position = vec4(position + offset, 1.0);
}
