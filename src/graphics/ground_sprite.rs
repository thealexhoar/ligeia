use ligeia_utils::rect::FloatRect;
use sfml::graphics::{Vertex};

use graphics::{Renderable};

pub struct GroundSprite {
    _radius: f32,
    _rect: FloatRect,
    _vertices: [Vertex; 4]
}


impl GroundSprite {
    pub fn new(origin_x: f32, origin_y: f32, width: f32, height: f32, tex_rect: &FloatRect) -> Self {
        let radius = ((width * width + height * height) as f32 * 0.25).sqrt();
        let mut sprite = Self {
            _radius: radius,
            _rect: FloatRect::new_square(origin_x - radius, origin_y - radius, radius * 2.),
            _vertices: [Vertex::default(); 4]
        };

        sprite.set_local_vertices(origin_x, origin_y, width, height);
        sprite.set_tex_rect(tex_rect);

        sprite
    }

    pub fn set_local_vertices(&mut self, origin_x: f32, origin_y: f32, width: f32, height: f32) {
        let h_width = width * 0.5;
        let h_height = height * 0.5;

        self._vertices[0].position.x = -h_width - origin_x;
        self._vertices[0].position.y = -h_height - origin_y;

        self._vertices[1].position.x = h_width - origin_x;
        self._vertices[1].position.y = -h_height - origin_y;

        self._vertices[2].position.x = h_width - origin_x;
        self._vertices[2].position.y = h_height - origin_y;

        self._vertices[3].position.x = -h_width - origin_x;
        self._vertices[3].position.y = h_height - origin_y;
    }

    pub fn set_tex_rect(&mut self, rect: &FloatRect) {
        self._vertices[0].tex_coords.x = rect.left;
        self._vertices[0].tex_coords.y = rect.top;

        self._vertices[1].tex_coords.x = rect.left + rect.width;
        self._vertices[1].tex_coords.y = rect.top;

        self._vertices[2].tex_coords.x = rect.left + rect.width;
        self._vertices[2].tex_coords.y = rect.top + rect.height;

        self._vertices[3].tex_coords.x = rect.left;
        self._vertices[3].tex_coords.y = rect.top + rect.height;
    }
}


impl Renderable for GroundSprite {
    fn radius(&self) -> f32 { self._radius }

    fn rect(&self) -> &FloatRect { &self._rect }

    fn vertices_needed(&self) -> usize { 4 }

    fn write_to_vertices(&self, x: f32, y: f32, theta: f32, camera_theta: f32, target: &mut [Vertex]) {
        for i in 0..4 {
            let local_x = self._vertices[i].position.x;
            let old_y = self._vertices[i].position.y;
            target[i].position.x = (local_x * camera_theta.cos() - old_y * camera_theta.sin()) + x;
            target[i].position.y = (local_x * camera_theta.sin() + old_y * camera_theta.cos()) + y;
            target[i].tex_coords = self._vertices[i].tex_coords;
        }
    }
}