use nphysics2d::world::World;
use std::cell::RefCell;
use std::rc::Rc;

use physics::consts::*;


pub type PhysicsWorld = World<f32>;

pub fn construct_world() -> PhysicsWorld {
    let mut world = PhysicsWorld::new();
    {
        let params = world.integration_parameters_mut();

        params.dt = PHYSICS_TIMESTEP;
        params.max_velocity_iterations = VELOCITY_ITERATIONS;
        params.max_position_iterations = POSITION_ITERATIONS;
    }

    world
}