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

use game::components::PhysicsObject;
use game::resources::{DeltaTime, PhysicsTimeAccumulator};
use physics::{COLLIDER_MARGIN, PhysicsWorld, PHYSICS_TIMESTEP};

pub struct PhysicsUpdate;

impl PhysicsUpdate {
    pub fn new() -> Self {
        Self {}
    }
}

impl<'a> System<'a> for PhysicsUpdate {
    type SystemData = (
        ReadExpect<'a, DeltaTime>,
        WriteExpect<'a, PhysicsTimeAccumulator>,
        WriteExpect<'a, PhysicsWorld>
    );
    fn run(
        &mut self,
        (
            delta_time,
            mut time_accumulator,
            mut world
        ): Self::SystemData
    ) {
        let mut temp_dt = delta_time.dt + time_accumulator.time;
        while temp_dt > PHYSICS_TIMESTEP {
            world.step();
            temp_dt -= PHYSICS_TIMESTEP;
        }
        (*time_accumulator.deref_mut()).time = temp_dt;
    }
}