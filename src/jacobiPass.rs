use crate::postProcessPass::PostProcessPass;
use crate::shader::ShaderProgram;

pub struct JacobiPass {
  shader: ShaderProgram
}

impl PostProcessPass for JacobiPass {
  fn render(&mut self) {
    self.shader.enable();
    self.shader.set_uniform_1i("u_velocity", 12);
    self.shader.set_uniform_1i("u_pressure", 0);

    unsafe {
      gl::DrawElements(gl::TRIANGLE_STRIP, 286, gl::UNSIGNED_INT, std::ptr::null());
    }
  }
}

impl JacobiPass {
  pub fn new() -> Self {
    JacobiPass {
      shader: ShaderProgram::new("shaders/divergence.vert.glsl", "shaders/jacobi.frag.glsl")
    }
  }
}