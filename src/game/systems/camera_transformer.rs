use specs::{Fetch, Join, ReadStorage, System, WriteStorage};

use game::components::{ScreenPosition, WorldPosition};
use graphics::ManagedCamera;

pub struct CameraTransformer;

impl<'a> System<'a> for CameraTransformer {
    type SystemData = (Fetch<'a, ManagedCamera>, ReadStorage<'a, WorldPosition>, WriteStorage<'a, ScreenPosition>);

    fn run(&mut self, (camera, world_pos, mut screen_pos): Self::SystemData) {
        for (world_pos, screen_pos) in (&world_pos, &mut screen_pos).join() {
            let (screen_x, screen_y) = camera.transform_world_point(world_pos.x, world_pos.y);
            screen_pos.x = screen_x;
            screen_pos.y = screen_y;
            screen_pos.theta = camera.transform_world_angle(world_pos.theta);
        }
    }
}
