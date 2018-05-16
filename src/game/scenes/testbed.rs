use specs::{DispatcherBuilder, World};
use std::cell::RefCell;
use std::rc::Rc;

use game::Scene;
use game::systems::*;
use graphics::{ShaderHandler, TextureHandler, Window};
use physics::PhysicsWorld;

pub fn testbed<'a>(
    shader_handler: Rc<RefCell<ShaderHandler<'a>>>,
    texture_handler: Rc<RefCell<TextureHandler>>,
    window: Rc<RefCell<Window>>,
    physics_world: Rc<RefCell<PhysicsWorld>>
) -> Scene<'a> {
    let world_renderer = WorldRenderer::new(shader_handler, texture_handler, window);
    let physics = Physics::new(physics_world.clone());
    let dispatchers = vec![
        Box::new(DispatcherBuilder::new()
            .with_thread_local(physics)
            .build()
        ),
        Box::new(DispatcherBuilder::new()
            .with(CameraTransformer, "camera_transformer", &[])
            .with(ScreenSort, "screen_sort", &["camera_transformer"])
            .with_thread_local(world_renderer)
            .with_thread_local(FPSPrint::new(60))
            .build()
        ),

    ];

    Scene::new_multi(
        dispatchers,
        None,
        None
    )
}