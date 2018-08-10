use gl;
use gl::types::*;

use std::ptr;
use std::mem::{
    size_of,
    transmute
};

use {
    Color,
    Framebuffer,
    ProjectionMatrix,
    Shader,
    Texture,
    Vertex, VERTEX_SIZE,
};


pub struct RenderTexture {
    _fbo: Framebuffer,
    _vao: u32,
    _vbo: u32,
    _vbo_size: usize,
    _size: (u32, u32),
    _sampler: u32,
    _default_projection: ProjectionMatrix,
}

impl RenderTexture {
    pub fn new(width: i32, height: i32) -> Self {
        let uwidth = width.abs() as u32;
        let uheight = height.abs() as u32;
        let framebuffer = Framebuffer::new(uwidth, uheight);
        let mut vao = 0;
        let mut vbo = 0;
        let mut sampler = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);

            gl::GenBuffers(1, &mut vbo);
            gl::BindVertexArray(0);

            gl::GenSamplers(1, &mut sampler);
            gl::SamplerParameteri(sampler, gl::TEXTURE_WRAP_S, gl::REPEAT as GLint);
            gl::SamplerParameteri(sampler, gl::TEXTURE_WRAP_T, gl::REPEAT as GLint);
            gl::SamplerParameteri(sampler, gl::TEXTURE_MAG_FILTER, gl::NEAREST as GLint);
            gl::SamplerParameteri(sampler, gl::TEXTURE_MIN_FILTER, gl::NEAREST as GLint);
        }

        let mut default_projection = ProjectionMatrix::identity();
        default_projection.set_to_ortho(width as f32, height as f32);

        Self{
            _fbo: framebuffer,
            _vao: vao,
            _vbo: vbo,
            _vbo_size: 0,
            _size: (uwidth, uheight),
            _sampler: sampler,
            _default_projection: default_projection
        }
    }

    pub fn draw_vertices(
        &mut self,
        vertices: &[Vertex],
        texture: &Texture,
        shader: &Shader,
        projection: Option<&ProjectionMatrix>
    ) {
        let vertex_count = vertices.len();
        let buffer_size = vertex_count * VERTEX_SIZE;

        let projection_matrix = match projection {
            Some(val) => *val,
            None      => self._default_projection
        };

        unsafe {
            self._fbo.bind();

            gl::Viewport(0, 0, self._size.0 as i32, self._size.1 as i32);

            gl::ActiveTexture(gl::TEXTURE0);
            shader.bind(&projection_matrix);
            texture.bind();
            gl::BindSampler(0, self._sampler);

            gl::BindVertexArray(self._vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, self._vbo);

            if buffer_size > self._vbo_size {
                self._vbo_size = buffer_size;
                gl::BufferData(
                    gl::ARRAY_BUFFER,
                    (buffer_size * size_of::<GLfloat>()) as GLsizeiptr,
                    transmute(&vertices[0]),
                    gl::STREAM_DRAW
                );
            }
            else {
                gl::BufferSubData(
                    gl::ARRAY_BUFFER,
                    0,
                    (buffer_size * size_of::<GLfloat>()) as GLsizeiptr,
                    transmute(&vertices[0])
                );
            }


            gl::DrawArrays(gl::TRIANGLES, 0, vertex_count as i32);

            gl::BindSampler(0, 0);
            Texture::unbind();
            Shader::unbind();
            Framebuffer::unbind();
        }
    }

    pub fn texture(&self) -> &Texture {
        self._fbo.texture()
    }

    pub fn clear(&mut self) {
        self.clear_with_rgba(0., 0., 0., 0.);
    }

    pub fn clear_with_color(&mut self, color: &Color) {
        self.clear_with_rgba(color.r, color.g, color.b, color.a);
    }

    pub fn clear_with_rgba(&mut self, r: f32, g: f32, b: f32, a: f32) {
        self._fbo.bind();
        unsafe {
            gl::ClearColor(r, g, b, a);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
        Framebuffer::unbind();
    }
}