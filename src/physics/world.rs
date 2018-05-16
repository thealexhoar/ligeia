use nphysics2d::world::World;
use std::cell::RefCell;
use std::rc::Rc;

use physics::consts::*;


pub type PhysicsWorld = World<f32>;

pub fn construct_world() -> Rc<RefCell<PhysicsWorld>> {
    let mut world = PhysicsWorld::new();
    world.set_timestep(PHYSICS_TIMESTEP);

    Rc::new(RefCell::new(world))
}