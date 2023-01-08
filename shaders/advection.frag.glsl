#version 120

uniform sampler2D u_inputTexture;
uniform sampler2D u_velocity;

varying vec2 v_uv;

void main() {
  vec2 prevUv = fract(v_uv - 0.016 * texture2D(u_velocity, v_uv).xy);

  gl_FragColor = texture2D(u_inputTexture, prevUv);
}