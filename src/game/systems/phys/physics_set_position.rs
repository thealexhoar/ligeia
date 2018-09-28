use specs::{Join, ParJoin, ReadExpect, ReadStorage, System, WriteStorage};

use game::components::{PhysicsHandle::*, PhysicsObject, WorldPosition};
use physics::{meters_to_pix, PhysicsWorld};

pub struct PhysicsSetPosition;

impl<'a> System<'a> for PhysicsSetPosition {
    type SystemData = (
        ReadExpect<'a, PhysicsWorld>,
        ReadStorage<'a, PhysicsObject>,
        WriteStorage<'a, WorldPosition>
    );

    fn run (&mut self, (world, phys_objects, mut world_positions): Self::SystemData) {
        for (phys_object, mut world_pos) in (&phys_objects, &mut world_positions).join() {
            match phys_object.body_handle {
                Uninitialized => {},
                Initialized(body_handle) => {
                    let rigid_body = world.rigid_body(body_handle).unwrap();
                    let body_isometry = rigid_body.position();

                    world_pos.x = meters_to_pix(body_isometry.translation.vector.x);
                    world_pos.y = meters_to_pix(body_isometry.translation.vector.y);
                    world_pos.theta = body_isometry.rotation.angle();
                }
            }
        }
    }
}