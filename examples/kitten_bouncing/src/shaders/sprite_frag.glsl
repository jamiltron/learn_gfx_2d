#version 150 core

in vec2 our_texture_coord;

out vec4 color;

uniform sampler2D texture_sampler;

void main() {
  color = texture(texture_sampler, our_texture_coord);
}
