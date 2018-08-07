use specs::{ReadExpect, System};
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

use game::components::ScreenPosition;
use ligeia_graphics::{ShaderHandler, TextureHandler, Vertex, Window};

pub struct ShadowRenderer{
    _shader_handler: Rc<RefCell<ShaderHandler>>,
    _texture_handler: Rc<RefCell<TextureHandler>>,
    _vertices: Vec<Vertex>,
    _window: Rc<RefCell<Window>>
}

impl ShadowRenderer {
    pub fn new(
        shader_handler: Rc<RefCell<ShaderHandler>>,
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


impl<'a> System<'a> for ShadowRenderer {
    type SystemData = (ReadExpect<'a, ScreenPosition>);

    fn run(&mut self, data: Self::SystemData) {
        
    }
}
