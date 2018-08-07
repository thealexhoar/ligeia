use gl;
use gl::types::*;
use std::ffi::CString;
use std::mem::{
    size_of,
    transmute
};
use std::ops::{Deref, Drop};
use std::ptr;
use std::str;

use VERTEX_SIZE;

//TODO: implement
pub struct Shader {
    _program_id: GLuint,
    _frag_data_location: String,
    _frag_texture_uniform: String,
    _vert_pos_name: String,
    _vert_color_name: String,
    _vert_tex_coords_name: String
}

impl Shader {
    pub fn new(
        vert_source: &str,
        frag_source: &str,
        frag_data_location: &str,
        frag_texture_uniform: &str,
        vert_pos_name: &str,
        vert_color_name: &str,
        vert_tex_coords_name: &str
    ) -> Self {
        let vs = Self::compile_shader(vert_source, gl::VERTEX_SHADER);
        let fs = Self::compile_shader(frag_source, gl::FRAGMENT_SHADER);
        let program = Self::link_program(vs, fs);

        Self {
            _program_id: program,
            _frag_data_location: frag_data_location.to_string(),
            _frag_texture_uniform: frag_texture_uniform.to_string(),
            _vert_pos_name: vert_pos_name.to_string(),
            _vert_color_name: vert_color_name.to_string(),
            _vert_tex_coords_name: vert_tex_coords_name.to_string()
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::UseProgram(self._program_id);
            gl::Uniform1i(gl::GetUniformLocation(self._program_id, CString::new(self._frag_texture_uniform.as_str()).unwrap().as_ptr()), 0);
            gl::BindFragDataLocation(self._program_id, 0, CString::new(self._frag_data_location.as_str()).unwrap().as_ptr());

            let pos_attr = gl::GetAttribLocation(self._program_id, CString::new(self._vert_pos_name.as_str()).unwrap().as_ptr());
            gl::EnableVertexAttribArray(pos_attr as GLuint);
            gl::VertexAttribPointer(
                pos_attr as GLuint,
                2,
                gl::FLOAT,
                gl::FALSE as GLboolean,
                (VERTEX_SIZE * size_of::<GLfloat>()) as GLsizei,
                ptr::null()
            );

            let color_attr = gl::GetAttribLocation(self._program_id, CString::new(self._vert_color_name.as_str()).unwrap().as_ptr());
            gl::EnableVertexAttribArray(color_attr as GLuint);
            gl::VertexAttribPointer(
                color_attr as GLuint,
                4,
                gl::FLOAT,
                gl::FALSE as GLboolean,
                (VERTEX_SIZE * size_of::<GLfloat>()) as GLsizei,
                ptr::null().add(2 * size_of::<GLfloat>())
            );

            let uv_attr = gl::GetAttribLocation(self._program_id, CString::new(self._vert_tex_coords_name.as_str()).unwrap().as_ptr());
            gl::EnableVertexAttribArray(uv_attr as GLuint);
            gl::VertexAttribPointer(
                uv_attr as GLuint,
                2,
                gl::FLOAT,
                gl::FALSE as GLboolean,
                (VERTEX_SIZE * size_of::<GLfloat>()) as GLsizei,
                ptr::null().add(6 * size_of::<GLfloat>())
            );
        }
    }

    pub fn program(&self) -> GLuint {
        self._program_id
    }

    pub fn unbind() {
        unsafe {
            gl::UseProgram(0);
        }
    }

    fn compile_shader(src: &str, shader_type: GLenum) -> GLuint {
        unsafe {
            let shader = gl::CreateShader(shader_type);

            // link source and attempt to compile
            let c_str = CString::new(src.as_bytes()).unwrap();
            gl::ShaderSource(shader, 1, &c_str.as_ptr(), ptr::null());
            gl::CompileShader(shader);

            // get the status of the compilation attempt
            let mut status = gl::FALSE as GLint;
            gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut status);

            // fail on error
            if status != (gl::TRUE as GLint) {
                // code copied from example online
                let mut len = 0;
                gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
                let mut buf = Vec::with_capacity(len as usize);
                buf.set_len((len as usize) - 1); // subtract 1 to skip the trailing null character
                gl::GetShaderInfoLog(
                    shader,
                    len,
                    ptr::null_mut(),
                    buf.as_mut_ptr() as *mut GLchar,
                );
                panic!(
                    "{}",
                    str::from_utf8(&buf)
                        .ok()
                        .expect("ShaderInfoLog not valid utf8")
                );
            }
            shader
        }
    }

    fn link_program(vs: GLuint, fs: GLuint) -> GLuint {
        unsafe {
            let program = gl::CreateProgram();
            gl::AttachShader(program, vs);
            gl::AttachShader(program, fs);
            gl::LinkProgram(program);

            // Get link status
            let mut status = gl::FALSE as GLint;
            gl::GetProgramiv(program, gl::LINK_STATUS, &mut status);

            // Fail on error
            if status != (gl::TRUE as GLint) {
                let mut len: GLint = 0;
                gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);
                let mut buf = Vec::with_capacity(len as usize);
                buf.set_len((len as usize) - 1); // subtract 1 to skip the trailing null character
                gl::GetProgramInfoLog(
                    program,
                    len,
                    ptr::null_mut(),
                    buf.as_mut_ptr() as *mut GLchar,
                );
                panic!(
                    "{}",
                    str::from_utf8(&buf)
                        .ok()
                        .expect("ProgramInfoLog not valid utf8")
                );
            }

            program
        }
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        //TODO: implement
    }
}
