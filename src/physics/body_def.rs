use physics::{BodyType, ColliderDef, SensorDef};

#[derive(Clone)]
pub struct BodyDef {
    pub x: f32,
    pub y: f32,
    pub theta: f32,
    pub angular_velocity: f32,
    pub linear_velocity: (f32, f32),
    pub density: f32,
    pub fixed_rotation: bool,
    pub body_type: BodyType,
    pub collider_def: ColliderDef,
    pub sensor_defs: Vec<SensorDef>
}

impl BodyDef {
    pub fn new(body_type: BodyType, collider_def: ColliderDef) -> Self {
        Self {
            x: 0.,
            y: 0.,
            theta: 0.,
            angular_velocity: 0.,
            linear_velocity: (0., 0.),
            density: 0.,
            fixed_rotation: false,
            body_type,
            collider_def,
            sensor_defs: Vec::new()
        }
    }

    pub fn new_dynamic(collider_def: ColliderDef) -> Self {
        Self::new(BodyType::Dynamic, collider_def)
    }

    pub fn new_kinematic(collider_def: ColliderDef) -> Self {
        Self::new(BodyType::Kinematic, collider_def)
    }

    pub fn new_static(collider_def: ColliderDef) -> Self {
        Self::new(BodyType::Static, collider_def)
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

    pub fn with_density(&mut self, density: f32) -> &mut Self {
        self.density = density;
        self
    }

    pub fn with_sensors(&mut self, sensor_defs: Vec<SensorDef>) -> &mut Self{
        self.sensor_defs = sensor_defs;
        self
    }

}