use specs::{
    Join, ReadStorage, ReadExpect, WriteExpect, System,
};

use ligeia_graphics::ManagedCamera;

use game::components::{CameraFollowFlag, WorldPosition};

pub struct CameraFollow;

impl<'a> System<'a> for CameraFollow {
    type SystemData = (
        ReadStorage<'a, CameraFollowFlag>,
        ReadStorage<'a, WorldPosition>,
        WriteExpect<'a, ManagedCamera>
    );

    fn run(&mut self, (camera_flag, world_pos, mut camera): Self::SystemData) {
        for (_, pos) in (&camera_flag, &world_pos).join() {
            camera.x = pos.x;
            camera.y = pos.y;
        }
    }
}