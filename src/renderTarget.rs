use gl::{self, RENDERBUFFER_BLUE_SIZE};

use crate::math;

#[derive(Default, Clone, Copy)]
struct RenderBuffer {
  framebuffer: u32,
  texture: u32
}

impl RenderBuffer {
    pub fn new(resolution: &math::Vec2) -> Self {
        let mut render_buffer = RenderBuffer::default();

        unsafe {
            gl::GenFramebuffers(1, &mut render_buffer.framebuffer);
            gl::GenTextures(1, &mut render_buffer.texture);
            gl::BindFramebuffer(gl::FRAMEBUFFER, render_buffer.framebuffer);
            gl::BindTexture(gl::TEXTURE_2D, render_buffer.texture);
            gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA16F as i32, resolution.x as i32, resolution.y as i32, 0, gl::RGBA, gl::FLOAT, std::ptr::null());
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
            gl::FramebufferTexture2D(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT0, gl::TEXTURE_2D, render_buffer.texture, 0);
        }

        render_buffer
    }
}

pub struct RenderTarget {
  index: usize,
  render_buffer: Vec<RenderBuffer>
}

impl RenderTarget {
  pub fn new(size: usize, resolution: &math::Vec2) -> Self {
    RenderTarget {
        index: 0,
        render_buffer: vec![0; size].into_iter().map(|_| RenderBuffer::new(resolution)).collect()
    }
  }

  pub fn bind(&mut self) {
    let i = (self.index + 1) % self.render_buffer.len();
    let iii = self.index % self.render_buffer.len();

    unsafe {
        gl::BindFramebuffer(gl::FRAMEBUFFER, self.render_buffer[i].framebuffer);
        gl::BindTexture(gl::TEXTURE_2D, self.render_buffer[iii].texture);
    }

    self.index += 1;
  }

  pub fn get_texture(&self) -> u32 {
    self.render_buffer[self.index % self.render_buffer.len()].texture
  }
}