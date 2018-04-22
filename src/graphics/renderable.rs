use sfml::graphics::Vertex;

use util::FloatRect;

pub trait Renderable {
    fn rect(&self) -> &FloatRect;
    fn vertices_needed(&self) -> usize;
    fn write_to_vertices(&self, x: f32, y: f32, theta: f32, camera_theta: f32, target: &mut [Vertex]);
}