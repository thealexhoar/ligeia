use nphysics2d::object::BodyStatus;

#[derive(Clone, Copy)]
pub enum BodyType {
    Dynamic,
    Kinematic,
    Static,
}

impl BodyType {
    pub fn to_body_status(&self) -> BodyStatus {
        match *self {
            Dynamic => BodyStatus::Dynamic,
            Kinematic => BodyStatus::Kinematic,
            Static => BodyStatus::Static
        }
    }
}