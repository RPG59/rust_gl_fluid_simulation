#version 120

uniform sampler2D u_velocity; // unit vector field

varying vec2 v_pos;

void main() {
  vec2 uv = (1. + v_pos) / 2.;
  vec2 texelSize = vec2(dFdx(uv.x), dFdy(uv.y));
  vec2 prevUv = fract(uv - texture2D(u_velocity, uv).xy * texelSize);

  gl_FragColor = vec4(texture2D(u_velocity, prevUv).xy * .99, 0., 1.);
}