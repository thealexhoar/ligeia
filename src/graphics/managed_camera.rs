

pub struct ManagedCamera {
    pub x: f32,
    pub y: f32,
    pub theta: f32,
    _width: f32,
    _height: f32,
    _radius: f32
}

impl ManagedCamera {
    pub fn new(x: f32, y: f32, theta: f32, width: f32, height: f32) -> Self {
        let r2 = (width * width + height * height) * 0.25;

        Self {
            x,
            y,
            theta,
            _width: width,
            _height: height,
            _radius: r2.sqrt()
        }
    }

    pub fn transform_world_point(&self, x:f32, y:f32) -> (f32, f32) {
        let trans_x = x - self.x;
        let trans_y = y - self.y;
        let s_t = self.theta.sin();
        let c_t = self.theta.cos();
        let rot_x = c_t * trans_x - s_t * trans_y;
        let rot_y = s_t * trans_x + c_t * trans_y;

        (rot_x, rot_y)
    }

    pub fn transform_world_angle(&self, theta:f32) -> f32 {
        theta - self.theta
    }

    pub fn overlaps_with(&self, x: f32, y: f32, radius: f32) -> bool {
        let dx = self.x - x;
        let dy = self.y - y;
        let dx2 = dx * dx;
        let dy2 = dy * dy;
        let rad_sum = self._radius + radius;

        return (dx2 + dy2) <= (rad_sum * rad_sum);
    }
}