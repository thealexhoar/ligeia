use specs::{System, WriteExpect};

use ligeia_graphics::ManagedCamera;

pub struct CameraUpdate;

impl<'a> System<'a> for CameraUpdate {
    type SystemData = WriteExpect<'a, ManagedCamera>;

    fn run(&mut self, mut camera: Self::SystemData) {
        camera.update();
    }
}