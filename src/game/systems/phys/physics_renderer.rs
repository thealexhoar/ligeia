use ncollide2d::world::CollisionGroups;
use ncollide2d::shape::*;
use specs::{Join, ReadExpect, ReadStorage, System, WriteStorage};
use std::any::Any;
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

use ligeia_graphics::{Vertex, Window};

use game::components::ScreenPosition;
use game::resources::*;
use physics::PhysicsWorld;


pub struct PhysicsRenderer {
    _vertices: Vec<Vertex>,
    _window: Rc<RefCell<Window>>
}

impl PhysicsRenderer {
    pub fn new(window: Rc<RefCell<Window>>) -> Self {
        Self {
            _vertices: Vec::with_capacity(1024),
            _window: window
        }
    }
}

impl<'a> System<'a> for PhysicsRenderer {
    type SystemData = (
        ReadExpect<'a, PhysicsWorld>,
        ReadExpect<'a, VerticesNeeded>,
        ReadExpect<'a, ScreenAABB>
    );

    fn run(&mut self, (world, vertices_needed, screen_aabb): Self::SystemData) {
        let mut collision_groups = CollisionGroups::new();
        collision_groups.enable_self_interaction();

        for collider in world
            .collision_world()
            .interferences_with_aabb(&screen_aabb.aabb, &collision_groups)
        {
            // note that an isometry represents a rotation THEN a translation
            let isometry = collider.position();
            let rotation: f32 = isometry.rotation.unwrap().re;
            let (x, y): (f32, f32) = unsafe { (
                    *isometry.translation.vector.get_unchecked(0, 0),
                    *isometry.translation.vector.get_unchecked(1, 0)
            )};
            // get shape
            // transform to fit screen
            // draw that mofok

            let shape = collider.shape();

            if shape.is_shape::<Ball<f32>>() {
                let ball: &Ball<f32> = shape.as_shape().unwrap();
                //TODO: add ball rendering
            }

            if shape.is_shape::<Cuboid<f32>>() {
                let cuboid: &Cuboid<f32> = shape.as_shape().unwrap();
                //TODO: add cuboid rendering
            }
        }
    }
}