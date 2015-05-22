#version 410

in vec3 position;
in vec3 offset;


void main() {
    gl_Position = vec4(position + offset, 1.0);
}
