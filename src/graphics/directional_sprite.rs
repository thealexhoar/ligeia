use ligeia_utils::rect::FloatRect;
use sfml::graphics::{Vertex};

use graphics::{Renderable};
use util::consts::{TWO_PI, radians_to_direction8};

#[derive(Clone, Copy, Debug)]
pub struct DirectionalSprite {
    _radius_2: f32,
    _rect: FloatRect,
    _vertices: [(f32, f32); 4],
    _tex_rects: [FloatRect; 8]
}

/*
Vertices in a sprite go clockwise as such:
    0-1
    | |
    3-2
*/

impl DirectionalSprite {
    pub fn new(origin_x: f32, origin_y: f32, width: f32, height: f32, tex_rects: &[FloatRect]) -> Self {
        let radius = ((width * width + height * height) as f32 * 0.25).sqrt();
        let mut rects = [FloatRect::new(0., 0., 0., 0.); 8];
        for i in 0..8 {
            rects[i] = tex_rects[i];
        }
        let mut sprite = Self {
            _radius_2: radius * radius,
            _rect: FloatRect::new_square(origin_x - radius, origin_y - radius, radius * 2.),
            _vertices: [(0., 0.); 4],
            _tex_rects: rects
        };

        sprite.set_local_vertices(origin_x, origin_y, width, height);

        sprite
    }

    pub fn set_vertices_as_rect(&mut self, width: f32, height: f32, centered: bool) {
        if centered {
            self.set_local_vertices(0., 0., width, height);
        }
            else {
                self.set_local_vertices(width * 0.5, height * 0.5, width, height);
            }
    }

    pub fn set_local_vertices(&mut self, origin_x: f32, origin_y: f32, width: f32, height: f32) {
        let h_width = width * 0.5;
        let h_height = height * 0.5;

        self._vertices[0] = (-h_width - origin_x, -h_height - origin_y);

        self._vertices[1] = (h_width - origin_x, -h_height - origin_y);

        self._vertices[2] = (h_width - origin_x, h_height - origin_y);

        self._vertices[3] = (-h_width - origin_x, h_height - origin_y);
    }

}

impl Renderable for DirectionalSprite {
    fn radius_2(&self) -> f32 { self._radius_2 }

    fn rect(&self) -> &FloatRect { &self._rect }

    fn vertices_needed(&self) -> usize { 4 }

    fn write_to_vertices(&self, x: f32, y: f32, theta: f32, camera_theta: f32, target: &mut [Vertex]) {
        let direction = radians_to_direction8(camera_theta);

        let tex_coords = &self._tex_rects[direction as usize];
        for i in 0..4 {
            let (local_x, local_y) = self._vertices[i];
            target[i].position.x = (local_x * theta.cos() - local_y * theta.sin()) + x;
            target[i].position.y = (local_x * theta.sin() + local_y * theta.cos()) + y;
            target[i].tex_coords.x = match i {
                0 | 3 => tex_coords.left,
                _     => tex_coords.left + tex_coords.width
            };
            target[i].tex_coords.y = match i {
                0 | 1 => tex_coords.top,
                _     => tex_coords.top + tex_coords.height
            };
        }
    }

}