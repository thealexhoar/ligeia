use sfml::graphics::{Color, PrimitiveType, RenderStates, RenderTarget, RenderWindow, Vertex, VertexArray, View};
use sfml::system::{Clock, Time, Vector2f};
use sfml::window::{ContextSettings, Event, Style, VideoMode};


pub struct Window {
    _clear_color: Color,
    _should_close: bool,
    _clock: Clock,
    _delta_time: Time,
    _window: RenderWindow,
}

impl Window {
    pub fn new(width: u32, height: u32, internal_width: u32, internal_height: u32, title: &str) -> Self {
        let mode = VideoMode { width, height, bits_per_pixel: 32 };
        let style = Style::CLOSE;
        let settings = ContextSettings::default();

        let mut window = RenderWindow::new(mode, title, style, &settings);
        window.set_vertical_sync_enabled(true);

        let mut view = View::new(Vector2f{x: 0., y: 0.}, Vector2f {x: internal_width as f32, y: internal_height as f32});
        window.set_view(&view);


        Self {
            _clear_color: Color::rgb(220, 111, 180),
            _should_close: false,
            _clock: Clock::start(),
            _delta_time: Time::seconds(0.),
            _window: window
        }
    }

    pub fn clear(&mut self) {
        //self._clear_color.r += 1;
        //self._clear_color.r %= 255;
        self._window.clear(&self._clear_color);
    }

    pub fn draw_vertex_array<'a>(&mut self, vertices: &VertexArray, states: RenderStates<'a, 'a, 'a>) {
        self._window.draw_vertex_array(vertices, states);
    }

    pub fn draw_vertices<'a>(&mut self, vertices: &[Vertex], states: RenderStates<'a, 'a, 'a>) {
        self._window.draw_primitives(vertices, PrimitiveType::Quads, states);
    }

    pub fn delta_time(&self) -> f32{
        self._delta_time.as_seconds()
    }

    pub fn display(&mut self) {
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