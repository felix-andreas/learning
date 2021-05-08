#version 330 core

layout(location = 0) out vec4 color;

void main () {
    color = vec4(0.5, gl_FragCoord.xy / 300, 1.0);
}
