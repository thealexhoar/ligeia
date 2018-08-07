use ligeia_utils::rect::FloatRect;
use Vertex;

pub trait Renderable {
    fn radius_2(&self) -> f32;
    fn rect(&self) -> &FloatRect;
    fn vertices_needed(&self) -> usize;
    fn write_to_vertices(&self, x: f32, y: f32, theta: f32, camera_theta: f32, target: &mut [Vertex]);
}