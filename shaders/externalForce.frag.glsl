#version 120

uniform vec2 u_mouse;
uniform vec2 u_direction;
uniform sampler2D u_velocity;

varying vec2 v_pos;

void main() {
  vec2 uv = v_pos / 2. + .5;
  float d = distance(uv, u_mouse);
  float strength = 1. / max(d, .001) / 1000.;
  strength *= clamp(dot(normalize(uv - u_mouse), normalize(-u_direction)), .0, 1.);

  gl_FragColor = vec4(texture2D(u_velocity, uv).xy + strength, 0., 1.);
}