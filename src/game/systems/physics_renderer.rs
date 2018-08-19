use specs::{Join, ReadExpect, ReadStorage, System, WriteStorage};
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

use ligeia_graphics::{Vertex, Window};

use game::components::ScreenPosition;
use game::resources::VerticesNeeded;
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
        ReadStorage<'a, ScreenPosition>,
    );

    fn run(&mut self, (world, vertices_needed, screen_pos): Self::SystemData) {
        for collider in world.colliders() {
            // note that an isometry represents a rotation THEN a translation
            let isometry = collider.position();
            let rotation: f32 = isometry.rotation.unwrap().re;
            let (x, y): (f32, f32) = unsafe { (
                    *isometry.translation.vector.get_unchecked(0, 0),
                    *isometry.translation.vector.get_unchecked(1, 0)
            )};
            // get shape
            // transform to fit screen
            // if onscreen, draw that mofo
        }

    }
}