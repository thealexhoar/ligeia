use gl;
use gl::types::*;

use std::ops::Drop;

use {
    Color,
    Texture
};


pub struct Framebuffer {
    _fbo_id: GLuint,
    _texture: Texture
}
// TODO: use a Renderbuffer internally
impl Framebuffer {
    pub fn new(width: u32, height: u32) -> Self {
        let mut fbo = 0;
        let texture = Texture::new_with_color(width, height, Color::new(0., 0., 0., 0.));
        unsafe {
            gl::GenFramebuffers(1, &mut fbo);
            gl::BindFramebuffer(gl::FRAMEBUFFER, fbo);

            gl::FramebufferTexture(
                gl::FRAMEBUFFER,
                gl::COLOR_ATTACHMENT0,
                texture.id(),
                0
            );

            gl::DrawBuffer(gl::COLOR_ATTACHMENT0);
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
        }

        Self {
            _fbo_id: fbo,
            _texture: texture
        }
    }

    pub fn texture(&self) -> &Texture {
        &self._texture
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, self._fbo_id);
        }
    }

    pub fn unbind() {
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
        }

    }
}

impl Drop for Framebuffer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteFramebuffers(1, &mut self._fbo_id);
        }
    }
}