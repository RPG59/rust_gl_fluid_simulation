#version 460

uniform sampler2D u_tex;

out vec4 color;
in vec2 v_pos;


void main() {
  vec2 uv = (v_pos + 1.) / 2.;
  color = vec4(texture2D(u_tex, uv).xy, 0., 1.);
}