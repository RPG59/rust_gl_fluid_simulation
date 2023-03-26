use crate::postProcessPass::PostProcessPass;
use crate::shader::ShaderProgram;

pub struct DivergencePass {
  shader: ShaderProgram,
}

/*
  divergence measuring the change in density of the fluid
  vector -> scalar
 */
impl PostProcessPass for DivergencePass {
  fn render(&mut self) {
    self.shader.enable();

    unsafe {
      gl::DrawElements(gl::TRIANGLE_STRIP, 286, gl::UNSIGNED_INT, std::ptr::null());
    }
  }
}

impl DivergencePass {
  pub fn new() -> Self {
    DivergencePass {
      shader: ShaderProgram::new("shaders/divergence.vert.glsl", "shaders/divergence.frag.glsl")
    }
  }
}