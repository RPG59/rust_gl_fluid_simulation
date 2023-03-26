#version 120
attribute vec3 position;

varying vec2 v_pos;

void main() {
  v_pos = position.xy;
  gl_Position = vec4(position, 1.);
}
