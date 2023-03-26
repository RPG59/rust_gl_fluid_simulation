#version 460

in vec2 v_pos;
out vec4 color;

uniform sampler2D u_velocity;
uniform sampler2D u_pressure;

void main() {
  vec2 uv = (v_pos + 1.) / 2.;
  vec2 texelSize = vec2(dFdx(uv.x), dFdy(uv.y));
  vec4 x0 = texture2D(u_pressure, vec2(uv.x - texelSize.x, uv.y));
  vec4 x1 = texture2D(u_pressure, vec2(uv.x + texelSize.x, uv.y));
  vec4 y0 = texture2D(u_pressure, vec2(uv.x, uv.y + texelSize.y));
  vec4 y1 = texture2D(u_pressure, vec2(uv.x, uv.y - texelSize.y));
  vec4 b = texture2D(u_velocity, uv);

  color = (x0 + x1 + y0 + y1 - b) / 4.;
}