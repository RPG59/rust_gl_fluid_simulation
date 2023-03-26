use crate::postProcessPass::PostProcessPass;
use crate::shader::ShaderProgram;
use gl;

pub struct Advection {
  pub shader: ShaderProgram,
}

impl PostProcessPass for Advection {
  fn render(&mut self) {
    self.shader.enable();
    
    unsafe {
      gl::DrawElements(gl::TRIANGLE_STRIP, 286, gl::UNSIGNED_INT, std::ptr::null());
    }
  }
}

impl Advection {
  pub fn new() -> Self {
    Advection { 
      shader: ShaderProgram::new( "shaders/divergence.vert.glsl", "shaders/advection.frag.glsl")
    }
  }
}

