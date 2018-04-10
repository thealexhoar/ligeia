use sfml::graphics::{IntRect, Vertex};
use graphics::TextureHandle;

pub struct Sprite {
    _vertices: [Vertex; 4],
    _texture_handle: TextureHandle,
    _texture_rect: IntRect
}

impl Sprite {
    pub fn new(texture: TextureHandle) -> Self {
        Self {

        }
    }

    pub fn with_subrect(texture: TextureHandle, rect: IntRect) -> Self {
        Self {}
    }
}