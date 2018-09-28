use ncollide2d::shape::ShapeHandle;
use ncollide2d::world::CollisionGroups;
use nphysics2d::object::Material;

#[derive(Clone)]
pub struct ColliderDef {
    pub local_x: f32,
    pub local_y: f32,
    pub local_rotation: f32,
    pub material: Material<f32>,
    pub shape: ShapeHandle<f32>,
    pub group: CollisionGroups
}

impl ColliderDef {
    pub fn new(shape: ShapeHandle<f32>) -> Self {
        Self {
            local_x: 0.,
            local_y: 0.,
            local_rotation: 0.,
            material: Material::new(0., 0.),
            shape,
            group: CollisionGroups::new()
        }
    }
}