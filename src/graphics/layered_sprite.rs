use bit_set::BitSet;
use sfml::graphics::{Vertex};
use std::collections::HashMap;

use graphics::{LayerDef, Renderable, ShaderHandle, TextureHandle};
use util::FloatRect;

#[derive(Clone, Debug)]
pub struct LayeredSprite {
    _base_vertices: [(f32, f32); 4],
    _rect: FloatRect,
    _tex_coords: HashMap<usize, FloatRect>,
    _layer_bits: BitSet,
    _vertex_count: usize
}

/*
Vertices in a sprite layer go clockwise as such:
    0-1
    | |
    3-2
*/

impl LayeredSprite {
    pub fn new(origin_x: f32, origin_y: f32, width: f32, height: f32, layers: &LayerDef) -> Self {
        let mut layer_bits = BitSet::new();
        let mut top_layer = 0;
        let mut layer_count = 0;
        let mut tex_coords: HashMap<usize, FloatRect> = HashMap::new();
        for (layer, rect) in &layers.layers {
            top_layer = top_layer.max(*layer);
            tex_coords.insert(*layer, *rect);
            layer_bits.insert(*layer);

            layer_count += 1;
        }

        let radius = ((width * width + height * height) as f32 * 0.25).sqrt();

        let mut layered_sprite = Self {
            _base_vertices: [(0., 0.); 4],
            _rect: FloatRect::new(origin_x - radius, origin_y - radius - (top_layer as f32), radius * 2., radius * 2. + top_layer as f32),
            _tex_coords: tex_coords,
            _layer_bits: layer_bits,
            _vertex_count: layer_count * 4
        };

        layered_sprite.set_local_vertices(origin_x, origin_y, width, height);

        layered_sprite
    }

    pub fn new_centered(width: f32, height: f32, layers: &LayerDef) -> Self {
        Self::new(0., 0., width, height, layers)
    }

    pub fn set_local_vertices(&mut self, origin_x: f32, origin_y: f32, width: f32, height: f32) {
        let h_width = width * 0.5;
        let h_height = height * 0.5;

        self._base_vertices[0] = (origin_x - h_width, origin_y - h_height);

        self._base_vertices[1] = (origin_x + h_width, origin_y - h_height);

        self._base_vertices[2] = (origin_x + h_width, origin_y + h_height);

        self._base_vertices[3] = (origin_x - h_width, origin_y + h_height);
    }
}


impl Renderable for LayeredSprite {
    fn rect(&self) -> &FloatRect { &self._rect }

    fn vertices_needed(&self) -> usize {
        self._vertex_count
    }

    fn write_to_vertices(&self, x: f32, y: f32, theta: f32, camera_theta: f32, target: &mut [Vertex]) {
        //let theta = theta - camera_theta;

        let mut index_counter = 0;
        for layer in &self._layer_bits {
            let rect = self._tex_coords[&layer];
            let index = index_counter * 4;
            index_counter += 1;
            for i in 0..4 {
                let index = index + i;
                let (local_x, local_y) = self._base_vertices[i];
                target[index].position.x = (local_x * camera_theta.cos() - local_y * camera_theta.sin()) + x;
                target[index].position.y = (local_x * camera_theta.sin() + local_y * camera_theta.cos()) + y - (layer as f32);
                target[index].tex_coords.x = match i {
                    0 | 3 => rect.left,
                    _     => rect.left + rect.width
                };
                target[index].tex_coords.y = match i {
                    0 | 1 => rect.top,
                    _     => rect.top + rect.height
                };
            }
        }
    }
}