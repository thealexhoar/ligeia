use specs::{Join, ReadExpect, ReadStorage, System, WriteExpect, WriteStorage};
use std::cell::RefCell;
use std::ops::DerefMut;
use std::rc::Rc;

use game::resources::{DeltaTime, PhysicsTimeAccumulator};
use physics::{PhysicsWorld, PHYSICS_TIMESTEP};

pub struct Physics;

impl Physics {
    pub fn new() -> Self { Self {} }
}

impl<'a> System<'a> for Physics {
    type SystemData = (
        ReadExpect<'a, DeltaTime>,
        WriteExpect<'a, PhysicsTimeAccumulator>,
        WriteExpect<'a, PhysicsWorld>
    );

    fn run (
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