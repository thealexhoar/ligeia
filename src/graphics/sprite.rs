use sfml::graphics::{FloatRect, Vertex};
use graphics::{Renderable, ShaderHandle, TextureHandle};

#[derive(Clone, Copy, Debug)]
pub struct Sprite {
    _radius: f32,
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
        let mut sprite = Self {
            _vertices: [Vertex::default(); 4],
            _radius: ((width * width + height * height) * 0.25 as f32).sqrt()
                   + ((origin_x * origin_x + origin_y * origin_y) * 0.25 as f32).sqrt()
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

        self._vertices[0].position.x = origin_x - h_width;
        self._vertices[0].position.y = origin_y - h_height;

        self._vertices[1].position.x = origin_x + h_width;
        self._vertices[1].position.y = origin_y - h_height;

        self._vertices[2].position.x = origin_x + h_width;
        self._vertices[2].position.y = origin_y + h_height;

        self._vertices[3].position.x = origin_x - h_width;
        self._vertices[3].position.y = origin_y + h_height;
    }

    pub fn get_world_vertices(&self, x: f32, y: f32, theta: f32) -> [Vertex; 4] {
        let mut world_verts = self._vertices;
        for i in 0..4 {
            let old_x = world_verts[i].position.x;
            let old_y = world_verts[i].position.y;
            world_verts[i].position.x = (old_x * theta.cos() - old_y * theta.sin()) + x;
            world_verts[i].position.y = (old_x * theta.sin() + old_y * theta.cos()) + y;
        }

        world_verts
    }

    pub fn get_shader_handle(&self) -> ShaderHandle {
        self._shader_handle
    }

    pub fn set_shader_handle(&mut self, handle: ShaderHandle) {
        self._shader_handle = handle
    }

    pub fn get_tex_handle(&self) -> TextureHandle {
        self._texture_handle
    }

    pub fn set_tex_handle(&mut self, handle: TextureHandle) {
        self._texture_handle = handle;
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

impl Renderable for Sprite {
    fn radius(&self) -> f32 { self._radius }

    fn vertices_needed(&self) -> usize { 4 }

    fn write_to_vertices(&self, x: f32, y: f32, theta: f32, camera_theta: f32, target: &mut [Vertex]) {
        for i in 0..4 {
            let old_x = self._vertices[i].position.x;
            let old_y = self._vertices[i].position.y;
            target[i].position.x = (old_x * theta.cos() - old_y * theta.sin()) + x;
            target[i].position.y = (old_x * theta.sin() + old_y * theta.cos()) + y;
            target[i].tex_coords = self._vertices[i].tex_coords;
        }
    }

}