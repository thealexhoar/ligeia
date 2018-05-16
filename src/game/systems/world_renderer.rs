use sfml::graphics::{PrimitiveType, RenderStates, Transformable, Vertex};
use sfml::graphics::Sprite as SFSprite;
use sfml::system::Vector2f;
use specs::{Join, ReadExpect, ReadStorage, System, WriteStorage};
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

use game::components::{ScreenPosition, WorldRenderable};
use game::resources::VerticesNeeded;
use graphics::{ShaderHandle, ShaderHandler, Sprite, TextureHandle, TextureHandler, Window};

pub struct WorldRenderer<'a> {
    _shader_handler: Rc<RefCell<ShaderHandler<'a>>>,
    _texture_handler: Rc<RefCell<TextureHandler>>,
    _vertices: Vec<Vertex>,
    _window: Rc<RefCell<Window>>
}

impl<'a> WorldRenderer<'a> {
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

impl<'a, 'b> System<'a> for WorldRenderer<'b> {
    type SystemData = (ReadExpect<'a, VerticesNeeded>, ReadStorage<'a, ScreenPosition>, ReadStorage<'a, WorldRenderable>);

    fn run(&mut self, (vertices_needed, screen_pos, world_renderable): Self::SystemData) {
        let vertices_needed = (*vertices_needed.deref()).world;
        if  vertices_needed >= self._vertices.len() {
            self._vertices.resize(vertices_needed, Vertex::default())
        }

        for (screen_pos, world_renderable) in (&screen_pos, &world_renderable).join() {
            if let Some(target_index) = screen_pos.vertex_index {
                let v = world_renderable.renderable.vertices_needed();
                world_renderable.renderable.write_to_vertices(
                    screen_pos.x,
                    screen_pos.y,
                    screen_pos.theta,
                    screen_pos.camera_theta,
                    &mut self._vertices[target_index..target_index+v]
                );
            }

        }

        let texture_handler = self._texture_handler.borrow();
        //let texture_ref = texture_handler.get_texture(sprite.get_tex_handle()).unwrap();
        let texture_ref = texture_handler.get_master_texture();

        let shader_handler = self._shader_handler.borrow();
        //let shader_ref = shader_handler.get_shader(sprite.get_shader_handle()).unwrap();
        let shader_ref = shader_handler.get_default().unwrap();

        let mut render_states = RenderStates::default();
        render_states.texture = Some(texture_ref);
        render_states.shader = Some(shader_ref);

        let mut window = self._window.borrow_mut();
        //let screen_verts = sprite.get_world_vertices(screen_pos.x, screen_pos.y, screen_pos.theta);

        window.draw_vertices(&self._vertices[0..vertices_needed], PrimitiveType::Quads, render_states);
    }
}