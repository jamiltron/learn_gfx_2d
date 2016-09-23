#version 150 core

in vec2 position;
in vec2 texture_coord;

uniform View {
  mat4 model;
  mat4 projection;
};

out vec2 our_texture_coord;

void main() {
  gl_Position = projection * model * vec4(position, 0.0, 1.0);
  our_texture_coord = vec2(texture_coord.x, 1.0 - texture_coord.y);
}
