use na::{
    geometry::{Isometry, Isometry2},
    Vector2
};
use nphysics2d::volumetric::Volumetric;
use specs::{
    BitSet, InsertedFlag, Join, ReadExpect, ReadStorage,
    ReaderId, RemovedFlag, System, WriteExpect, WriteStorage
};
use std::cell::RefCell;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;

use game::components::{PhysicsHandle, PhysicsObject};
use game::resources::{DeltaTime, PhysicsTimeAccumulator};
use physics::{COLLIDER_MARGIN, PhysicsWorld, PHYSICS_TIMESTEP};
use util::PI;

pub struct PhysicsAddRemove {
    _inserted_id: ReaderId<InsertedFlag>,
    _removed_id: ReaderId<RemovedFlag>,
    _inserted: BitSet,
    _removed: BitSet
}

impl PhysicsAddRemove {
    pub fn new(storage: &mut WriteStorage<PhysicsObject>) -> Self {
        Self {
            _inserted_id: storage.track_inserted(),
            _removed_id: storage.track_removed(),
            _inserted: BitSet::new(),
            _removed: BitSet::new()
        }
    }
}

impl<'a> System<'a> for PhysicsAddRemove {
    type SystemData = (
        WriteExpect<'a, PhysicsWorld>,
        WriteStorage<'a, PhysicsObject>
    );

    fn run (&mut self, (mut world, mut phys_objects): Self::SystemData) {
        self._inserted.clear();
        self._removed.clear();
        phys_objects.populate_inserted(&mut self._inserted_id, &mut self._inserted);
        phys_objects.populate_removed(&mut self._removed_id, &mut self._removed);

        for (mut phys_object, _) in (&mut phys_objects, &self._inserted).join() {
            let body_def = &phys_object.body_def;

            // create the body
            // create the collider
            // overwrite the phys object

            let body_isometry = Isometry2::new(
                Vector2::x() * body_def.x + Vector2::y() * body_def.y,
                body_def.theta// / PI * 180.
            );
            let body_handle = world.add_rigid_body(
                body_isometry,
                body_def.collider_def.shape.inertia(1.0),
                body_def.collider_def.shape.center_of_mass()
            );

            {
                let mut rigid_body = world.rigid_body_mut(body_handle).unwrap();
                rigid_body.set_status(body_def.body_type.to_body_status());
                let (vx, vy) = body_def.linear_velocity;
                rigid_body.set_linear_velocity(
                    Vector2::x() * vx + Vector2::y() * vy
                );
                rigid_body.set_angular_velocity(body_def.angular_velocity);
            }

            let collider_def = &body_def.collider_def;

            let collider_handle = world.add_collider(
                COLLIDER_MARGIN,
                collider_def.shape.clone(),
                body_handle,
                Isometry2::new(
                    Vector2::x() * collider_def.local_x + Vector2::y() * collider_def.local_y,
                    collider_def.local_rotation
                ),
                collider_def.material.clone()
            );

            {
                let collision_world = world.collision_world_mut();
                collision_world.set_collision_groups(collider_handle, collider_def.group);
            }


            let sensor_handles = body_def.sensor_defs.iter()
                .map(|ref sensor_def| {
                    let (x, y) = sensor_def.local_position;
                    world.add_sensor(
                        sensor_def.shape.clone(),
                        body_handle,
                        Isometry2::new(
                            Vector2::x() * x + Vector2::y() * y,
                            sensor_def.local_rotation
                        )
                    )
                })
                .collect::<Vec<_>>();

            //TODO: do something with colliders/sensors
            phys_object.body_handle = PhysicsHandle::Initialized(body_handle);
        }

        for (phys_object, _) in (&mut phys_objects, &self._removed).join() {
            //TODO: handle removal
        }
    }

}