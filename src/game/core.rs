use graphics::Window;

pub struct Core {
    _window: Window
}

impl Core {
    pub fn new(width: u32, height: u32, title: &str) -> Self {
        Self {
            _window: Window::new(width, height, title)
        }
    }

    pub fn update(&mut self) {
        // scene.update_logic()
        // scene.update_renderables()
        // for r in renderables:
        //   batcher.draw(r)
        self._window.process_events();
        self._window.clear();
        self._window.display();
    }

    pub fn should_close(&self) -> bool {
        self._window.should_close()
    }
}