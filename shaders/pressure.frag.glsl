#version 460

in vec2 v_pos;
out vec4 color;

uniform sampler2D u_velocity; // 0
uniform sampler2D u_pressure; // 1

void main() {
  vec2 uv = (v_pos + 1.) / 2.;
  vec2 texelSize = vec2(dFdx(uv.x), dFdy(uv.y));
  float x0 = texture2D(u_pressure, uv - vec2(texelSize.x, 0.)).x;
  float x1 = texture2D(u_pressure, uv + vec2(texelSize.x, 0.)).x;
  float y0 = texture2D(u_pressure, uv - vec2(0., texelSize.y)).x;
  float y1 = texture2D(u_pressure, uv + vec2(0., texelSize.y)).x;

  color = vec4(texture2D(u_velocity, uv).xy + vec2(x1 - x0, y1 - y0) / 2., 0., 1.);
  // color = vec4(texture2D(u_pressure, uv).xy, 0., 1.);
  // color = vec4((x1- x0 + y1 - y0), 0., 0., 1.);
}