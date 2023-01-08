#version 120

uniform sampler2D devergenceSampler;

varying vec2 v_pos;

float bilerp(float a, float b, float c, float d, float s, float t) {
  float x = mix(a, b, t);
  float y = mix(c, d, t);

  return mix(x, y, s);
}

void main() {
  vec2 uv = (v_pos + 1.) / 2.;
  vec2 texelSize = vec2(dFdx(uv.x), dFdy(uv.y));
  float newDevergence = texture2D(devergenceSampler, uv + vec2(texelSize.x, 0.)).x; // substruct HEIGHT; WIDTH

  newDevergence += texture2D(devergenceSampler, uv - vec2(texelSize.x, 0.)).x;
  newDevergence += texture2D(devergenceSampler, uv + vec2(0., texelSize.y)).x;
  newDevergence += texture2D(devergenceSampler, uv - vec2(0., texelSize.y)).x;
  newDevergence /= 4.;

  gl_FragColor = vec4(newDevergence, 0., 0., 0.);
}