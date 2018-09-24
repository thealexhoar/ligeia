use gl;
use gl::types::*;

pub enum PrimitiveType {
    TRIANGLES,
    LINES
}

impl PrimitiveType {
    pub fn to_gluint(&self) -> GLuint {
        unsafe {
            match self {
                PrimitiveType::TRIANGLES => gl::TRIANGLES,
                PrimitiveType::LINES     => gl::LINES
            }
        }
    }
}