use ligeia_utils::rect::FloatRect;

use ligeia_utils::PI;

pub struct ManagedCamera {
    pub x: f32,
    pub y: f32,
    pub theta: f32,
    _last_x: f32,
    _last_y: f32,
    _last_theta: f32,
    _width: f32,
    _height: f32,
    _rect: FloatRect,
    _radius_2: f32,
}

impl ManagedCamera {
    pub fn new(x: f32, y: f32, theta: f32, width: f32, height: f32) -> Self {
        let radius = ((width * width + height * height) as f32 * 0.25).sqrt();


        Self {
            x,
            y,
            theta,
            _last_x: x,
            _last_y: y,
            _last_theta: theta,
            _width: width,
            _height: height,
            _rect: FloatRect::new(-width * 0.5, -height * 0.5, width, height),
            _radius_2: radius * radius
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
        self.theta + theta
    }

    pub fn overlaps_with(&self, x: f32, y: f32, radius_2: f32, rect: &FloatRect) -> bool {
        /*let dx = self.x - x;
        let dy = self.y - y;
        let d2 = dx * dx + dy * dy;
        if d2 <= self._radius_2 + radius_2 {
            self._rect.intersects_rect(x, y, rect)
        }
        else {
            false
        }*/

        self._rect.intersects_rect(x, y, rect)
    }

    pub fn overlaps_with_world(&self, x: f32, y: f32, radius_2: f32, rect: &FloatRect) -> bool {
        /*let dx = self.x - x;
        let dy = self.y - y;
        let d2 = dx * dx + dy * dy;
        if d2 <= self._radius_2 + radius_2 {
            self._rect.intersects_rect(x, y, rect)
        }
        else {
            false
        }*/
        let (x, y) = self.transform_world_point(x, y);

        self._rect.intersects_rect(x, y, rect)
    }

    pub fn up(&self) -> (f32, f32) {
        let theta = -self.theta + PI * 0.5;
        (theta.cos(), theta.sin())
    }

    pub fn right(&self) -> (f32, f32) {
        let theta = -self.theta;
        (theta.cos(), theta.sin())
    }

    pub fn moving(&self) -> bool {
        self._last_theta != self.theta
            || self._last_x != self.x
            || self._last_y != self.y
    }

    pub fn update(&mut self) {
        self._last_x = self.x;
        self._last_y = self.y;
        self._last_theta = self.theta;
    }
}