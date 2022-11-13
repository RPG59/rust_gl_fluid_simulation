#version 120

uniform sampler2D u_tex;

varying vec2 v_uv;


void main() {
  gl_FragColor = vec4(texture2D(u_tex, v_uv).x, 0., 0., 1.);
}