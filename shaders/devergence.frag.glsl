#version 120

uniform vec2 u_mouse; // 0..1
uniform sampler2D devergenceSampler;
uniform vec2 u_px;

varying vec2 v_pos;

float bilerp(float a, float b, float c, float d, float s, float t) {
  float x = mix(a, b, t);
  float y = mix(c, d, t);

  return mix(x, y, s);
}

void main() {
  vec2 uv = (v_pos + 1.) / 2.;
  float newDevergence = texture2D(devergenceSampler, uv + vec2(u_px.x, 0.)).x; // substruct HEIGHT; WIDTH

  newDevergence += texture2D(devergenceSampler, uv - vec2(u_px.x, 0.)).x;
  newDevergence += texture2D(devergenceSampler, uv + vec2(0., u_px.y)).x;
  newDevergence += texture2D(devergenceSampler, uv - vec2(0., u_px.y)).x;
  newDevergence /= 4.;

  float dst = distance(uv, u_mouse);
  float mouseData = mix(1., 0., dst > 0.009 ? 1. : dst);

  gl_FragColor = vec4(newDevergence + mouseData, 0., 0., 0.);
}