use specs::{
    BitSet, InsertedFlag, Join, ReadExpect, ReadStorage,
    ReaderId, RemovedFlag, System, WriteExpect, WriteStorage
};
use std::cell::RefCell;
use std::ops::DerefMut;
use std::rc::Rc;

use game::components::PhysicsObject;
use game::resources::{DeltaTime, PhysicsTimeAccumulator};
use physics::{PhysicsWorld, PHYSICS_TIMESTEP};

pub struct Physics {
    _inserted_id: ReaderId<InsertedFlag>,
    _removed_id: ReaderId<RemovedFlag>,
    _inserted: BitSet,
    _removed: BitSet
}

impl Physics {
    pub fn new(storage: &mut WriteStorage<PhysicsObject>) -> Self {
        Self {
            _inserted_id: storage.track_inserted(),
            _removed_id: storage.track_removed(),
            _inserted: BitSet::new(),
            _removed: BitSet::new()
        }
    }
}

impl<'a> System<'a> for Physics {
    type SystemData = (
        ReadExpect<'a, DeltaTime>,
        WriteExpect<'a, PhysicsTimeAccumulator>,
        WriteExpect<'a, PhysicsWorld>,
        WriteStorage<'a, PhysicsObject>
    );

    fn run (
        &mut self,
        (
            delta_time,
            mut time_accumulator,
            mut world,
            mut phys_objects
        ): Self::SystemData
    ) {
        phys_objects.populate_inserted(&mut self._inserted_id, &mut self._inserted);
        phys_objects.populate_removed(&mut self._removed_id, &mut self._removed);

        for phys_object in (&mut phys_objects, &self._inserted).join() {
            //TODO: handle insertion

            // create the body
            // create the colliders
            // overwrite the phys object
        }

        for phys_object in (&mut phys_objects, &self._removed).join() {
            //TODO: handle removal
        }

        let mut temp_dt = delta_time.dt + time_accumulator.time;
        while temp_dt > PHYSICS_TIMESTEP {
            world.step();
            temp_dt -= PHYSICS_TIMESTEP;
        }
        (*time_accumulator.deref_mut()).time = temp_dt;
    }
}