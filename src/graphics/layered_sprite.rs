use bit_set::BitSet;
use sfml::graphics::{FloatRect, Vertex};
use std::collections::HashMap;

use graphics::{LayerDef, Renderable, ShaderHandle, TextureHandle};

#[derive(Clone, Copy, Debug)]
pub struct LayeredSprite {
    _base_vertices: [(f32, f32); 4],
    _tex_coords: HashMap<usize, FloatRect>,
    _layers: BitSet,
    _vertex_count: usize,
    _radius: f32
}

/*
Vertices in a sprite layer go clockwise as such:
    0-1
    | |
    3-2
*/

impl LayeredSprite {
    pub fn new(origin_x: f32, origin_y: f32, width: f32, height: f32, layers: &LayerDef) -> Self {
        let mut layered_sprite = Self {
            _base_vertices: [(0., 0.); 4],
            
        }
    }

    pub fn new_centered(width: f32, height: f32, layers: &LayerDef) -> Self {
        Self::new(0., 0., width, height, layers)
    }
}


impl Renderable for LayeredSprite {
    fn radius(&self) -> f32 {

    }

    fn vertices_needed(&self) -> usize {

    }

    fn write_to_vertices(&self, x: f32, y: f32, theta: f32, camera_theta: f32, target: &mut [Vertex]) {

    }
}