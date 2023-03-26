#version 460  
layout(location = 0) in vec3 position;

out vec2 v_pos;

void main() {
  v_pos = position.xy;
  gl_Position = vec4(position, 1.);
}
