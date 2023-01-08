use crate::postProcessPass::PostProcessPass;
use crate::math::Vec2;
use crate::shader::ShaderProgram;

pub struct ExternalForce {
  shader: ShaderProgram,
  prev_force_data: Vec2
}

impl PostProcessPass for ExternalForce {
  fn render(&mut self) {
    self.shader.enable();

    unsafe {
      gl::DrawElements(gl::TRIANGLE_STRIP, 286, gl::UNSIGNED_INT, std::ptr::null());
    }
  }
}

impl ExternalForce {
  pub fn new() -> Self {
    ExternalForce {
      shader: ShaderProgram::new("shaders/devergence.vert.glsl", "shaders/externalForce.frag.glsl"),
      prev_force_data: Vec2::default(),
    }
  }

  pub fn set_force(&mut self, data: &Vec2) {
    self.shader.enable();
    self.shader.set_uniform_vec2("u_mouse", data);
    self.shader.set_uniform_vec2("u_direction", &self.prev_force_data.sub(data));
    self.prev_force_data = data.clone();
  }
}