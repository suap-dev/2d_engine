pub mod shape;
pub mod world;

use glium::implement_vertex;
use nalgebra_glm::Vec2;

#[derive(Clone, Copy)]
struct Vertex {
    position: [f32; 2],
}
impl From<Vec2> for Vertex {
    fn from(value: Vec2) -> Self {
        Self {
            position: value.into(),
        }
    }
}
implement_vertex!(Vertex, position);
