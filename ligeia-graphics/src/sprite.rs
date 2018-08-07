use ligeia_utils::rect::FloatRect;

use {Renderable, Vertex};

#[derive(Clone, Copy, Debug)]
pub struct Sprite {
    _radius_2: f32,
    _rect: FloatRect,
    _vertices: [Vertex; 4]
}

/*
Vertices in a sprite go clockwise as such:
    0-1
    | |
    3-2
*/

impl Sprite {
    pub fn new(origin_x: f32, origin_y: f32, width: f32, height: f32, tex_rect: &FloatRect) -> Self {
        let radius = ((width * width + height * height) as f32 * 0.25).sqrt();
        let mut sprite = Self {
            _radius_2: radius * radius,
            _rect: FloatRect::new_square(origin_x - radius, origin_y - radius, radius * 2.),
            _vertices: [Vertex::default(); 4]
        };

        sprite.set_local_vertices(origin_x, origin_y, width, height);
        sprite.set_tex_rect(tex_rect);

        sprite
    }

    pub fn new_centered(width: f32, height: f32) -> Self {
        Self::new(0., 0., width, height, &FloatRect::new(0., 0., 1., 1.))
    }

    pub fn new_with_origin(origin_x: f32, origin_y: f32, width: f32, height: f32) -> Self {
        Self::new(origin_x, origin_y, width, height, &FloatRect::new(0., 0., 1., 1.))
    }

    pub fn new_centered_with_rect(width: f32, height: f32, tex_rect: &FloatRect) -> Self {
        Self::new(width * 0.5, height * 0.5, width, height, tex_rect)
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

        self._vertices[0].set_position_xy(-h_width - origin_x, -h_height - origin_y);
        self._vertices[1].set_position_xy(h_width - origin_x, -h_height - origin_y);
        self._vertices[2].set_position_xy(h_width - origin_x, h_height - origin_y);
        self._vertices[3].set_position_xy(-h_width - origin_x, h_height - origin_y);

    }

    pub fn set_tex_rect(&mut self, rect: &FloatRect) {
        self._vertices[0].set_tex_coords_uv(rect.left, rect.top);
        self._vertices[1].set_tex_coords_uv(rect.left + rect.width, rect.top);
        self._vertices[2].set_tex_coords_uv(rect.left + rect.width, rect.top + rect.height);
        self._vertices[3].set_tex_coords_uv(rect.left, rect.top + rect.height);
    }
}

impl Renderable for Sprite {
    fn radius_2(&self) -> f32 { self._radius_2 }

    fn rect(&self) -> &FloatRect { &self._rect }

    fn vertices_needed(&self) -> usize { 4 }

    fn write_to_vertices(&self, x: f32, y: f32, theta: f32, camera_theta: f32, target: &mut [Vertex]) {
        for i in 0..4 {
            let local_x = self._vertices[i].position_x();
            let old_y = self._vertices[i].position_y();
            target[i].set_position_xy(
                (local_x * theta.cos() - old_y * theta.sin()) + x,
                (local_x * theta.sin() + old_y * theta.cos()) + y
            );
            let tex_coords = self._vertices[i].tex_coords();
            target[i].set_tex_coords(&tex_coords);
        }
    }

}