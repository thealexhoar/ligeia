
use physics::{BodyType, ColliderDef};

#[derive(Clone)]
pub struct BodyDef {
    pub x: f32,
    pub y: f32,
    pub theta: f32,
    pub angular_velocity: f32,
    pub linear_velocity: (f32, f32),
    pub body_type: BodyType,
}

impl BodyDef {
    fn new(body_type: BodyType) -> Self {
        Self {
            x: 0.,
            y: 0.,
            theta: 0.,
            angular_velocity: 0.,
            linear_velocity: (0., 0.),
            body_type,
        }
    }

    pub fn new_dynamic() -> Self {
        Self::new(BodyType::Dynamic)
    }

    pub fn new_kinematic() -> Self {
        Self::new(BodyType::Kinematic)
    }

    pub fn new_static() -> Self {
        Self::new(BodyType::Static)
    }

    pub fn with_pos(&mut self, x: f32, y: f32) -> &mut Self {
        self.x = x;
        self.y = y;
        self
    }

    pub fn with_rotation(&mut self, theta: f32) -> &mut Self {
        self.theta = theta;
        self
    }


    pub fn with_angular_velocity(&mut self, velocity: f32) -> &mut Self {
        self.angular_velocity = velocity;
        self
    }

    pub fn with_linear_velocity(&mut self, vx: f32, vy: f32) -> &mut Self {
        self.linear_velocity = (vx, vy);
        self
    }

}