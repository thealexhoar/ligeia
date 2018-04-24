use sfml::graphics::{Color, PrimitiveType, RenderStates, RenderTarget, RenderTexture, RenderWindow, Vertex, VertexArray, View};
use sfml::system::{Clock, Time, Vector2f};
use sfml::window::{ContextSettings, Event, Style, VideoMode};

use graphics::{BASIC_VERTS, ShaderHandle, ShaderHandler};

pub struct Window {
    _clear_color: Color,
    _should_close: bool,
    _clock: Clock,
    _delta_time: Time,
    _target_texture: RenderTexture,
    _window: RenderWindow,
}

impl Window {
    pub fn new(width: u32, height: u32, internal_width: u32, internal_height: u32, title: &str) -> Self {
        let mode = VideoMode { width, height, bits_per_pixel: 32 };
        let style = Style::CLOSE;
        let settings = ContextSettings::default();


        let target_view = View::new(Vector2f{x: 0., y: 0.}, Vector2f {x: internal_width as f32, y: internal_height as f32});
        //let mut target_texture = RenderTexture::new(internal_width, internal_height, false).unwrap();
        //let mut target_texture = RenderTexture::new(internal_width * 2, internal_height * 2, false).unwrap();
        let mut target_texture = RenderTexture::new(width, height, false).unwrap();
        target_texture.set_view(&target_view);

        let mut window = RenderWindow::new(mode, title, style, &settings);
        window.set_vertical_sync_enabled(true);

        let window_view = View::new(Vector2f{x: 0.5, y: 0.5}, Vector2f {x: 1., y: -1.});
        window.set_view(&window_view);
        //window.set_view(&target_view);


        Self {
            _clear_color: Color::rgb(220, 111, 180),
            _should_close: false,
            _clock: Clock::start(),
            _delta_time: Time::seconds(0.),
            _target_texture: target_texture,
            _window: window
        }
    }

    pub fn clear(&mut self) {
        //self._clear_color.r += 1;
        //self._clear_color.r %= 255;
        self._target_texture.clear(&self._clear_color);
        self._window.clear(&Color::BLACK);
    }

    pub fn draw_vertex_array<'a>(&mut self, vertices: &VertexArray, states: RenderStates<'a, 'a, 'a>) {
        self._target_texture.draw_vertex_array(vertices, states);
        //self._window.draw_vertex_array(vertices, states);
    }

    pub fn draw_vertices<'a>(&mut self, vertices: &[Vertex], states: RenderStates<'a, 'a, 'a>) {
        self._target_texture.draw_primitives(vertices, PrimitiveType::Quads, states);
        //self._window.draw_primitives(vertices, PrimitiveType::Quads, states);
    }

    pub fn delta_time(&self) -> f32{
        self._delta_time.as_seconds()
    }

    pub fn display<'a>(&mut self, shader_handler: &ShaderHandler<'a>) {

        self._target_texture.display();

        let mut render_states = RenderStates::default();
        render_states.shader = shader_handler.get_default();
        render_states.texture = Some(self._target_texture.texture());

        self._window.draw_primitives(&BASIC_VERTS, PrimitiveType::Quads, render_states);

        self._window.display();
        self._delta_time = self._clock.restart();
    }

    pub fn process_events(&mut self) {
        while let Some(next_event) = self._window.poll_event() {
            match next_event {
                Event::Closed => { self._should_close = true },
                _             => {}
            }
        }
    }

    pub fn should_close(&self) -> bool {
        self._should_close
    }

}