use gl;
use gl::types::*;
use sdl2::video::Window as SDLWindow;
use sdl2::{
    Sdl,
    video::{GLContext, GLProfile, SwapInterval},
    VideoSubsystem
};
use std::mem::{
    size_of,
    transmute
};
use std::ptr;
use std::time::Duration;

use ligeia_utils::Stopwatch;

use {
    BASIC_VERTS,
    Color,
    Framebuffer,
    ProjectionMatrix,
    SCREEN_VERTS,
    Shader,
    ShaderHandle, ShaderHandler,
    Texture,
    Vertex, VERTEX_SIZE
};
use PrimitiveType;

//TODO: define a null shader
pub struct Window {
    _clear_color: Color,
    _delta_time: f32,
    _gl_context: GLContext,
    _should_close: bool,
    _default_projection: ProjectionMatrix,
    _size: (u32, u32),
    _internal_size: (u32, u32),
    _fbo: Framebuffer,
    _vao: u32,
    _vbo: u32,
    _sampler: u32,
    _fbo_sampler: u32,
    _vbo_size: usize,
    _stopwatch: Stopwatch,
    _window: SDLWindow
}

impl Window {
    // construct a new window
    pub fn new(
        video: &VideoSubsystem,
        width: u32,
        height: u32,
        internal_width: u32,
        internal_height: u32,
        use_vsync: bool,
        title: &str
    ) -> Self {
        let gl_attr = video.gl_attr();
        gl_attr.set_double_buffer(true);
        gl_attr.set_context_profile(GLProfile::Core);
        gl_attr.set_context_version(3, 3);

        let window = video
            .window(title, width, height)
            .position_centered()
            .opengl()
            .build()
            .unwrap();

        let context = window.gl_create_context().unwrap();
        gl::load_with(|s| video.gl_get_proc_address(s) as *const _);

        debug_assert_eq!(gl_attr.context_profile(), GLProfile::Core);
        debug_assert_eq!(gl_attr.context_version(), (3, 3));

        let framebuffer = Framebuffer::new(internal_width, internal_height);
        let mut vao = 0;
        let mut vbo = 0;
        let mut sampler = 0;
        let mut fbo_sampler = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);

            gl::GenBuffers(1, &mut vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

            gl::BufferData(
                gl::ARRAY_BUFFER,
                (4 * size_of::<GLfloat>()) as GLsizeiptr,
                ptr::null(),
                gl::STREAM_DRAW
            );


            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);

            gl::GenSamplers(1, &mut sampler);
            gl::SamplerParameteri(sampler, gl::TEXTURE_WRAP_S, gl::REPEAT as GLint);
            gl::SamplerParameteri(sampler, gl::TEXTURE_WRAP_T, gl::REPEAT as GLint);
            gl::SamplerParameteri(sampler, gl::TEXTURE_MAG_FILTER, gl::NEAREST as GLint);
            gl::SamplerParameteri(sampler, gl::TEXTURE_MIN_FILTER, gl::NEAREST as GLint);

            gl::GenSamplers(1, &mut fbo_sampler);
            gl::SamplerParameteri(fbo_sampler, gl::TEXTURE_WRAP_S, gl::REPEAT as GLint);
            gl::SamplerParameteri(fbo_sampler, gl::TEXTURE_WRAP_T, gl::REPEAT as GLint);
            gl::SamplerParameteri(fbo_sampler, gl::TEXTURE_MAG_FILTER, gl::NEAREST as GLint);
            gl::SamplerParameteri(fbo_sampler, gl::TEXTURE_MIN_FILTER, gl::NEAREST as GLint);
        }

        if (use_vsync) {
            video.gl_set_swap_interval(SwapInterval::VSync);
        }
        else {
            video.gl_set_swap_interval(SwapInterval::Immediate);
        }

        let mut default_projection = ProjectionMatrix::identity();
        //default_projection.set_to_ortho(-1. * (internal_width / 2) as f32, -1. * (internal_height / 2) as f32, internal_width as f32, internal_height as f32);
        default_projection.set_to_ortho(internal_width as f32, internal_height as f32);

        Self {
            //_clear_color: Color::new(0.9, 0.7, 0.8, 1.),
            _clear_color: Color::new(0., 0., 0., 1.),
            _delta_time: 1. / 60.,
            _gl_context: context,
            _should_close: false,
            _default_projection: default_projection,
            _size: (width, height),
            _internal_size: (internal_width, internal_height),
            _fbo: framebuffer,
            _vao: vao,
            _vbo: vbo,
            _sampler: sampler,
            _fbo_sampler: fbo_sampler,
            _vbo_size: 4,
            _stopwatch: Stopwatch::new(),
            _window: window
        }

    }

    // clear the window
    pub fn clear(&mut self) {
        unsafe {
            gl::ClearColor(
                self._clear_color.r,
                self._clear_color.g,
                self._clear_color.b,
                self._clear_color.a
            );
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    }

    pub fn clear_with_rgba(&mut self, r: f32, g: f32, b: f32, a: f32) {
        unsafe {
            gl::ClearColor(r, g, b, a);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    }

    pub fn clear_fbo(&mut self) {
        unsafe {
            self._fbo.bind();
            gl::ClearColor(0., 0., 0., 0.);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            Framebuffer::unbind();
        }
    }


    //prep for rendering
    pub fn begin(&mut self) {
        unsafe {
            gl::Disable(gl::CULL_FACE);
            gl::Disable(gl::DEPTH_TEST);
            gl::Enable(gl::TEXTURE_2D);
            gl::Enable(gl::BLEND);

            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        }
    }


    // draw a set of vertices to the framebuffer
    pub fn draw_vertices(
        &mut self,
        vertices: &[Vertex],
        texture: &Texture,
        shader: &Shader,
        projection: Option<&ProjectionMatrix>,
        primitive_type: PrimitiveType
    ) {
        let vertex_count = vertices.len();
        let buffer_size = vertex_count * VERTEX_SIZE;

        let projection_matrix = match projection {
            Some(val) => *val,
            None      => self._default_projection
        };

        unsafe {
            self._fbo.bind();
            gl::Viewport(0, 0, self._internal_size.0 as i32, self._internal_size.1 as i32);
            //gl::ClearColor(0., 0., 0., 0.);
            //gl::Clear(gl::COLOR_BUFFER_BIT);

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

            gl::DrawArrays(primitive_type.to_gluint(), 0, vertex_count as i32);

            gl::BindSampler(0, 0);
            Texture::unbind();
            Shader::unbind();
            Framebuffer::unbind();
        }
    }

    pub fn draw_single_texture(&mut self, shader: &Shader, texture: &Texture) {
        let vertices = &SCREEN_VERTS;
        let vertex_count = 4;
        let buffer_size = vertex_count * VERTEX_SIZE;

        unsafe {
            self.clear();

            gl::Viewport(0, 0, self._size.0 as i32, self._size.1 as i32);
            gl::ActiveTexture(gl::TEXTURE0);
            shader.bind(&ProjectionMatrix::identity());
            texture.bind();
            gl::BindSampler(0, self._sampler);

            gl::BindVertexArray(self._vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, self._vbo);

            gl::BufferSubData(
                gl::ARRAY_BUFFER,
                0,
                (buffer_size * size_of::<GLfloat>()) as GLsizeiptr,
                transmute(&vertices[0])
            );

            let indices: Vec<GLuint> = vec![
                0, 1, 2,
                0, 2, 3
            ];

            gl::DrawElements(
                gl::TRIANGLES,
                6,
                gl::UNSIGNED_INT,
                transmute(&indices[0])
            );

            gl::BindSampler(0, 0);
            Texture::unbind();
            Shader::unbind();
        }
    }

    pub fn draw_framebuffer(&mut self, shader: &Shader) {
        let vertices = &SCREEN_VERTS;
        let vertex_count = 4;
        let buffer_size = vertex_count * VERTEX_SIZE;

        unsafe {
            self.clear();

            gl::Viewport(0, 0, self._size.0 as i32, self._size.1 as i32);
            gl::ActiveTexture(gl::TEXTURE0);
            shader.bind(&ProjectionMatrix::identity());
            self._fbo.texture().bind();
            gl::BindSampler(0, self._fbo_sampler);

            gl::BindVertexArray(self._vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, self._vbo);

            gl::BufferSubData(
                gl::ARRAY_BUFFER,
                0,
                (buffer_size * size_of::<GLfloat>()) as GLsizeiptr,
                transmute(&vertices[0])
            );

            let indices: Vec<GLuint> = vec![
                0, 1, 2,
                0, 2, 3
            ];

            gl::DrawElements(
                gl::TRIANGLES,
                6,
                gl::UNSIGNED_INT,
                transmute(&indices[0])
            );

            gl::BindSampler(0, 0);
            Texture::unbind();
            Shader::unbind();

            self.clear_fbo();
        }
    }

    // return the time since the last frame, in seconds
    pub fn delta_time(&self) -> f32 {
        self._delta_time
    }

    // render the framebuffer and update delta time
    pub fn display(&mut self) {
        self._window.gl_swap_window();
        let duration = self._stopwatch.reset();
        self._delta_time = (duration.as_secs() as f64 + duration.subsec_nanos() as f64 * 1e-9) as f32;
    }


    pub fn resize(&mut self, width: u32, height: u32) {}

    // close the window
    pub fn close(&self) -> bool {
        //TODO: implement?
        true
    }

}