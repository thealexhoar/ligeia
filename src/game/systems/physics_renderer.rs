use specs::{Join, ReadExpect, ReadStorage, System, WriteStorage};
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

use game::components::ScreenPosition;
use game::resources::VerticesNeeded;
use ligeia_graphics::{Vertex, Window};
use physics::PhysicsWorld;


pub struct PhysicsRenderer {
    _phys_world: Rc<RefCell<PhysicsWorld>>,
    _vertices: Vec<Vertex>,
    _window: Rc<RefCell<Window>>
}

impl PhysicsRenderer {
    pub fn new(phys_world: Rc<RefCell<PhysicsWorld>>, window: Rc<RefCell<Window>>) -> Self {
        Self {
            _phys_world: phys_world,
            _vertices: Vec::with_capacity(1024),
            _window: window
        }
    }
}

impl<'a> System<'a> for PhysicsRenderer {
    type SystemData = (ReadExpect<'a, VerticesNeeded>, ReadStorage<'a, ScreenPosition>,);

    fn run(&mut self, (vertices_needed, screen_pos): Self::SystemData) {
        let world = self._phys_world.borrow();
        let collision_world = world.collision_world();

    }
}