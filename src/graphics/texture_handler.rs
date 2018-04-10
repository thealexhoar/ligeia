use sfml::graphics::Texture;
use sfml::system::SfBox;
use std::collections::HashMap;
use std::ops::Deref;

pub type TextureHandle = u32;

pub struct TextureHandler {
    _handle_gen: TextureHandle,
    _textures: HashMap<TextureHandle, SfBox<Texture>>
}

impl TextureHandler {
    pub fn new() -> Self {
        Self {
            _handle_gen: 0,
            _textures: HashMap::new()
        }
    }

    pub fn load_texture(&mut self, filename: &str) -> Option<TextureHandle> {
        match Texture::from_file(filename) {
            Some(t_box) => {
                self._textures.insert(self._handle_gen, t_box);
                self._handle_gen += 1;
                Some(self._handle_gen - 1)
            },
            None        => None
        }
    }

    pub fn unload_texture(&mut self, handle: TextureHandle) -> bool {
        match self._textures.remove(&handle) {
            Some(_) => true,
            None    => false
        }
    }

    pub fn get_texture(&self, handle: TextureHandle) -> Option<&Texture> {
        match self._textures.get(&handle) {
            Some(t_box) => Some(t_box.deref()),
            None        => None
        }
    }
}