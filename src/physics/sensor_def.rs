use ncollide2d::shape::ShapeHandle;

#[derive(Clone)]
pub struct SensorDef {
    pub local_position: (f32, f32),
    pub local_rotation: f32,
    pub shape: ShapeHandle<f32>,
}

impl SensorDef {
    pub fn new(shape: ShapeHandle<f32>) -> Self {
        Self {
            local_position: (0., 0.),
            local_rotation: 0.,
            shape
        }
    }
}