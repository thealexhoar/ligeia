use gl;
use gl::types::*;
use std::ops::Drop;

use ligeia_utils::ImageLoader;

use Color;

pub struct Texture {
     _texture_id: GLuint
}

impl Texture {
    pub fn new_blank(width: u32, height: u32) -> Self {
        Self::new_with_color(width, height, Color::WHITE)
    }

    pub fn new_with_color(width: u32, height: u32, color: Color) -> Self {
        let mut tex: u32 = 0;
        let size = width * height * 4;
        let mut data: Vec<f32> = Vec::with_capacity(size as usize);
        for i in 0..size/4  {
            data.push(color.r);
            data.push(color.g);
            data.push(color.b);
            data.push(color.a);
        }
        unsafe {
            let data_ptr = (&data[..]).as_ptr();

            gl::GenTextures(1, &mut tex);
            gl::BindTexture(gl::TEXTURE_2D, tex);
            gl::TexImage2D(
                gl::TEXTURE_2D,      // target type
                0,                   // level of detail (0 is base)
                gl::RGBA as i32,     // internal format
                width as i32,        // width
                height as i32,       // height
                0,                   // border, must be 0
                gl::RGBA,            // format of pixel data (GL_RGB, GL_RGBA, etc)
                gl::FLOAT,           // type of the pixel data (see openGL spec)
                data_ptr as *const _ // pixel data in memory
            );
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
        Self {
            _texture_id: tex
        }
    }


    pub fn new_from_file(filename: &str) -> Self {
        let image_loader = ImageLoader::open(filename).unwrap();

        Self::new_from_memory(
            image_loader.width(),
            image_loader.height(),
            image_loader.pixel_data()
        )
    }

    pub fn new_from_memory(width: u32, height: u32, pixel_data: Vec<f32>) -> Self {
        let mut tex: u32 = 0;
        if pixel_data.len() >= (width * height * 4) as usize {
            unsafe {
                let data_ptr = (&pixel_data[..]).as_ptr();

                gl::GenTextures(1, &mut tex);
                gl::BindTexture(gl::TEXTURE_2D, tex);
                gl::TexImage2D(
                    gl::TEXTURE_2D,      // target type
                    0,                   // level of detail (0 is base)
                    gl::RGBA as i32,     // internal format
                    width as i32,        // width
                    height as i32,       // height
                    0,                   // border, must be 0
                    gl::RGBA,            // format of pixel data (GL_RGB, GL_RGBA, etc)
                    gl::FLOAT,           // type of the pixel data (see openGL spec)
                    data_ptr as *const _ // pixel data in memory
                );
                gl::BindTexture(gl::TEXTURE_2D, 0);
            }
        }

        Self {
            _texture_id: tex
        }
    }

    pub fn id(&self) -> GLuint {
        self._texture_id
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self._texture_id);
        }
    }

    pub fn unbind() {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, &mut self._texture_id);
        }
    }
}