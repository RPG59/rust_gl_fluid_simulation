#version 120
attribute vec3 position;

varying vec2 v_uv;

void main() {
  v_uv = (position.xy + 1.) / 2.;
  gl_Position = vec4(position, 1.);
}
