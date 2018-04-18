use sfml::graphics::Vertex;

pub trait Renderable {
    fn radius(&self) -> f32;
    fn vertices_needed(&self) -> usize;
    fn write_to_vertices(&self, x: f32, y: f32, theta: f32, camera_theta: f32, target: &mut [Vertex]);
}