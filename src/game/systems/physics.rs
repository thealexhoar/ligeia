use specs::{Join, ReadExpect, ReadStorage, System, WriteExpect, WriteStorage};
use std::cell::RefCell;
use std::ops::DerefMut;
use std::rc::Rc;

use game::resources::{DeltaTime, PhysicsTimeAccumulator};
use physics::{PhysicsWorld, PHYSICS_TIMESTEP};

pub struct Physics {
    _phys_world: Rc<RefCell<PhysicsWorld>>
}

impl Physics {
    pub fn new(phys_world: Rc<RefCell<PhysicsWorld>>) -> Self {
        Self { _phys_world: phys_world}
    }
}

impl<'a> System<'a> for Physics {
    type SystemData = (
        ReadExpect<'a, DeltaTime>,
        WriteExpect<'a, PhysicsTimeAccumulator>
    );

    fn run(&mut self, (delta_time, mut time_accumulator): Self::SystemData) {
        let mut world = self._phys_world.borrow_mut();
        let mut temp_dt = delta_time.dt + time_accumulator.time;
        while temp_dt > PHYSICS_TIMESTEP {
            world.step();
            temp_dt -= PHYSICS_TIMESTEP;
        }
        (*time_accumulator.deref_mut()).time = temp_dt;
    }
}