use gl::types::*;

use Vector2f;

// column-major ordering
static M11: usize = 0;
static M21: usize = 1;
static M31: usize = 2;
static M12: usize = 3;
static M22: usize = 4;
static M32: usize = 5;
static M13: usize = 6;
static M23: usize = 7;
static M33: usize = 8;

// 3x3 projection matrix
#[derive(Clone, Copy, Debug)]
pub struct ProjectionMatrix {
    pub position: Vector2f,
    _data: [GLfloat; 9],
}

impl ProjectionMatrix {
    pub fn identity() -> Self {
        Self {
            position: Vector2f::new(0., 0.),
            _data: [
                1., 0., 0.,
                0., 1., 0.,
                0., 0., 1.
            ]
        }
    }

    pub fn set_to_ortho(&mut self, width: f32, height: f32) {
        self._data[M11] = 2. / width;
        self._data[M21] = 0.;
        self._data[M31] = 0.;
        self._data[M12] = 0.;
        self._data[M22] = 2. / height;
        self._data[M32] = 0.;
        self._data[M13] = 0.;
        self._data[M23] = 0.;
        self._data[M33] = 1.;
    }

    pub fn combine(&mut self) {
        //TODO: implement
    }

    pub fn data(&self) -> &[GLfloat] {
        &self._data
    }
}