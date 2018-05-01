use specs::{DispatcherBuilder, World};
use std::cell::RefCell;
use std::rc::Rc;

use game::Scene;
use game::systems::*;
use graphics::{ShaderHandler, TextureHandler, Window};

pub fn testbed<'a>(
    shader_handler: Rc<RefCell<ShaderHandler<'a>>>,
    texture_handler: Rc<RefCell<TextureHandler>>,
    window: Rc<RefCell<Window>>
) -> Scene<'a> {
    let world_renderer = WorldRenderer::new(shader_handler, texture_handler, window);
    let dispatcher = DispatcherBuilder::new()
        .add(CameraTransformer, "camera_transformer", &[])
        .add(ScreenSort, "screen_sort", &["camera_transformer"])
        .add_thread_local(world_renderer)
        .add_thread_local(FPSPrint::new(60))
        .build();

    Scene::new(
        Box::new(dispatcher),
        None,
        None
    )
}