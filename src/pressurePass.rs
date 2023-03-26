use crate::postProcessPass::PostProcessPass;
use crate::shader::ShaderProgram;

pub struct PressurePass {
  shader: ShaderProgram
}

impl PostProcessPass for PressurePass {
  fn render(&mut self) {
    self.shader.enable();
    self.shader.set_uniform_1i("u_velocity", 0);
    self.shader.set_uniform_1i("u_pressure", 12);

    unsafe {
      gl::DrawElements(gl::TRIANGLE_STRIP, 286, gl::UNSIGNED_INT, std::ptr::null());
    }
  }
}

impl PressurePass {
  pub fn new() -> Self {
    PressurePass {
      shader: ShaderProgram::new("shaders/divergence.vert.glsl", "shaders/pressure.frag.glsl")
    }
  }
}