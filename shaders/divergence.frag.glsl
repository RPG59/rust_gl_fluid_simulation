#version 120

uniform sampler2D divergenceSampler;

varying vec2 v_pos;

float bilerp(float a, float b, float c, float d, float s, float t) {
  float x = mix(a, b, t);
  float y = mix(c, d, t);

  return mix(x, y, s);
}

void main() {
  vec2 uv = (v_pos + 1.) / 2.;
  vec2 texelSize = vec2(dFdx(uv.x), dFdy(uv.y));
  float x0 = texture2D(divergenceSampler, uv + vec2(texelSize.x, 0.)).x;
  float x1 = texture2D(divergenceSampler, uv - vec2(texelSize.x, 0.)).x;
  float y0 = texture2D(divergenceSampler, uv + vec2(0., texelSize.y)).y;
  float y1 = texture2D(divergenceSampler, uv - vec2(0., texelSize.y)).y;

  gl_FragColor = vec4((x1 - x0 + y1 - y0) / 2.);
}