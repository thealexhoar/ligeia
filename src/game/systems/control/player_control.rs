use na::{
    geometry::Isometry2,
    Vector2
};
use nphysics2d::algebra::Force2;
use specs::{
    world::EntitiesRes,
    Entities, Join, System, ReadExpect,
    ReadStorage,  WriteExpect, WriteStorage,
};
use std::ops::{Deref, DerefMut};

use ligeia_graphics::ManagedCamera;

use game::components::{
    PhysicsHandle, PhysicsObject, PlayerFlag
};
use game::resources::{
    ControllerBox,
    DeltaTime,
    MajorEntities
};
use input::{Axis, Button, Controller};
use physics::PhysicsWorld;
use util::{true_atan, normalize};


pub struct PlayerControl;

impl<'a> System<'a> for PlayerControl {
    type SystemData = (
        ReadExpect<'a, ControllerBox>,
        ReadExpect<'a, DeltaTime>,
        ReadExpect<'a, EntitiesRes>,
        ReadStorage<'a, PhysicsObject>,
        WriteExpect<'a, PhysicsWorld>,
        ReadStorage<'a, PlayerFlag>,
        WriteExpect<'a, MajorEntities>,
        WriteExpect<'a, ManagedCamera>
    );

    fn run(
        &mut self,
        (
            controller,
            delta_time,
            entities,
            physics_objects,
            mut physics_world,
            player_flags,
            mut major_entities,
            mut camera
        ): Self::SystemData
    ) {
        //TODO: implement
        for (e, phys_object, _) in (&entities, &physics_objects, &player_flags).join() {
            major_entities.player = Some(e);

            let dx = controller.axis(Axis::MOVE_X);
            let dy = controller.axis(Axis::MOVE_Y);

            let camera_delta = controller.axis(Axis::CAMERA_X);

            camera.theta += 2. * 0.2 * delta_time.dt * camera_delta;

            if let PhysicsHandle::Initialized(handle) = phys_object.body_handle {
                let phys_body = physics_world.rigid_body_mut(handle).unwrap();


                if dx != 0. || dy != 0. {
                    let (ndx, ndy) = normalize(dx, dy);
                    let (up_x, up_y) = camera.up();
                    let (right_x, right_y) = camera.right();

                    let dx = right_x * ndx + up_x * ndy;
                    let dy = right_y * ndx + up_y * ndy;

                    phys_body.activate();
                    phys_body.set_linear_velocity(Vector2::new(dx * 5., dy * 5.));
                    let new_pos = Isometry2::new(
                        phys_body.position().translation.vector,
                        true_atan(dx, dy)
                    );
                    phys_body.set_position(new_pos);
                }
                else {
                    phys_body.set_linear_velocity(Vector2::new(0., 0.));
                }
            }
        }

    }
}