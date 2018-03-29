

pub struct ManagedCamera {
    _x: f32,
    _y: f32,
    _theta: f32,
    _width: f32,
    _height: f32,
    _r2: f32
}

impl ManagedCamera {
    pub fn new(x: f32, y: f32, theta: f32, width: f32, height: f32) -> Self {
        let r2 = (width * width + height * height) * 0.25;

        Self {
            _x: x,
            _y: y,
            _theta: theta,
            _width: width,
            _height: height,
            _r2: r2
        }
    }

    pub fn transform_world_point(&self, x:f32, y:f32) -> (f32, f32) {
        let trans_x = x - self._x;
        let trans_y = y - self._y;
        let s_t = self._theta.sin();
        let c_t = self._theta.cos();
        let rot_x = c_t * trans_x - s_t * trans_y;
        let rot_y = s_t * trans_x + c_t * trans_y;

        (rot_x, rot_y)
    }

    pub fn transform_world_angle(&self, theta:f32) -> f32 {
        theta - self._theta
    }

    pub fn get_sqr_radius(&self) -> f32 {
        self._r2
    }
}