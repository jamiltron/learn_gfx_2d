#version 150 core

in vec2 position;
in vec3 color;

out vec4 our_color;

void main() {
  our_color = vec4(color, 1.0);
  gl_Position = vec4(position, 0.0, 1.0);
}
