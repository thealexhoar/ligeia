use specs::{Fetch, System};
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

use game::components::ScreenPosition;
use graphics::{ShaderHandler, TextureHandler, Vertex, Window};

pub struct ShadowRenderer<'a> {
    _shader_handler: Rc<RefCell<ShaderHandler<'a>>>,
    _texture_handler: Rc<RefCell<TextureHandler>>,
    _vertices: Vec<Vertex>,
    _window: Rc<RefCell<Window>>
}

impl<'a> ShadowRenderer<'a> {
    pub fn new(
        shader_handler: Rc<RefCell<ShaderHandler<'a>>>,
        texture_handler: Rc<RefCell<TextureHandler>>,
        window: Rc<RefCell<Window>>
    ) -> Self {
        Self {
            _shader_handler: shader_handler,
            _texture_handler: texture_handler,
            _vertices: Vec::with_capacity(1024),
            _window: window
        }
    }
}


impl<'a, 'b> System<'a> for ShadowRenderer<'b> {
    type SystemData = (Fetch<'a, ScreenPosition>);

    fn run(&mut self, data: Self::SystemData) {
        
    }
}
