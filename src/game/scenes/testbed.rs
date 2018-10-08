use specs::{DispatcherBuilder, World};
use std::cell::RefCell;
use std::rc::Rc;

use game::Scene;
use game::systems::*;
use ligeia_graphics::{ShaderHandler, TextureHandler, Window};
use physics::PhysicsWorld;

pub fn testbed<'a>(
    shader_handler: &Rc<RefCell<ShaderHandler>>,
    texture_handler: &Rc<RefCell<TextureHandler>>,
    window: &Rc<RefCell<Window>>,
    world: &mut World
) -> Scene<'a> {
    let world_renderer = WorldRenderer::new(
        Rc::clone(shader_handler),
        Rc::clone(texture_handler),
        Rc::clone(window)
    );
    let physics_renderer = PhysicsRenderer::new(
        Rc::clone(shader_handler),
        Rc::clone(texture_handler),
        Rc::clone(window)
    );
    let dispatchers = vec![
        Box::new(DispatcherBuilder::new()
            .with(CameraUpdate, "camera_update", &[])
            .with_thread_local(PhysicsAddRemove::new(&mut world.write_storage()))
            .with_thread_local(PhysicsUpdate::new())
            .build()
        ),
        Box::new(DispatcherBuilder::new()
            .with(PlayerControl, "player_control", &[])
            .build()
        ),
        Box::new(DispatcherBuilder::new()
            .with(PhysicsSetPosition, "physics_set_position", &[])
            .with(CameraFollow, "camera_follow", &["physics_set_position"])
            .with(CameraTransformer, "camera_transformer", &["physics_set_position", "camera_follow"])
            .with(ScreenSort, "screen_sort", &["camera_transformer"])
            .with_thread_local(world_renderer)
            .with_thread_local(physics_renderer)
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