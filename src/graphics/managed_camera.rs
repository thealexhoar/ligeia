use util::FloatRect;

pub struct ManagedCamera {
    pub x: f32,
    pub y: f32,
    pub theta: f32,
    _width: f32,
    _height: f32,
    _broadphase: FloatRect,
    _radius: f32,
}

impl ManagedCamera {
    pub fn new(x: f32, y: f32, theta: f32, width: f32, height: f32) -> Self {
        let radius = ((width * width + height * height) as f32 * 0.25).sqrt();

        let fuzz = 0.;
        let h_fuzz = 0.5 * fuzz;

        Self {
            x,
            y,
            theta,
            _width: width,
            _height: height,
            _broadphase: FloatRect::new(-width * 0.5 - h_fuzz, -height * 0.5 - h_fuzz, width + fuzz, height + fuzz),
            _radius: radius + fuzz
        }
    }

    pub fn transform_world_point(&self, x: f32, y: f32) -> (f32, f32) {
        let trans_x = x - self.x;
        let trans_y = y - self.y;
        let s_t = self.theta.sin();
        let c_t = self.theta.cos();
        let rot_x = c_t * trans_x - s_t * trans_y;
        let rot_y = s_t * trans_x + c_t * trans_y;

        (rot_x, rot_y)
    }

    pub fn transform_world_angle(&self, theta: f32) -> f32 {
        self.theta - theta
    }

    pub fn overlaps_with(&self, x: f32, y: f32, rect: &FloatRect) -> bool {
        self._broadphase.intersects_at(x, y, rect)
    }
}