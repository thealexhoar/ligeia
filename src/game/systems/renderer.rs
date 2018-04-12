use sfml::graphics::{RenderStates, Transformable};
use sfml::graphics::Sprite as SFSprite;
use sfml::system::Vector2f;
use specs::{Fetch, Join, ReadStorage, System, WriteStorage};
use std::cell::RefCell;
use std::rc::Rc;


use game::components::{ScreenPosition, SpriteRenderable};
use graphics::{ShaderHandle, ShaderHandler, Sprite, TextureHandle, TextureHandler, Window};

pub struct Renderer<'a> {
    _shader_handler: Rc<RefCell<ShaderHandler<'a>>>,
    _texture_handler: Rc<RefCell<TextureHandler>>,
    _window: Rc<RefCell<Window>>
}

impl<'a> Renderer<'a> {
    pub fn new(
        shader_handler: Rc<RefCell<ShaderHandler<'a>>>,
        texture_handler: Rc<RefCell<TextureHandler>>,
        window: Rc<RefCell<Window>>
    ) -> Self {
        Self {
            _shader_handler: shader_handler,
            _texture_handler: texture_handler,
            _window: window
        }
    }
}

impl<'a, 'b> System<'a> for Renderer<'b> {
    type SystemData = (ReadStorage<'a, ScreenPosition>, ReadStorage<'a, SpriteRenderable>);

    fn run(&mut self, (screen_pos, sprite_renderable): Self::SystemData) {

        for (screen_pos, sprite_renderable) in (&screen_pos, &sprite_renderable).join() {
            let sprite: Sprite = sprite_renderable.sprite;

            let texture_handler = self._texture_handler.borrow();
            let texture_ref = texture_handler.get_texture(sprite.get_tex_handle()).unwrap();

            let shader_handler = self._shader_handler.borrow();
            let shader_ref = shader_handler.get_shader(sprite.get_shader_handle()).unwrap();

            let mut render_states = RenderStates::default();
            render_states.texture = Some(texture_ref);
            render_states.shader = Some(shader_ref);

            let mut window = self._window.borrow_mut();
            let screen_verts = sprite.get_world_vertices(screen_pos.x, screen_pos.y, screen_pos.theta);

            window.draw_vertices(&screen_verts, render_states);
        }
    }
}