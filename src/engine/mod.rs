pub mod world;
pub mod shape;

use std::f32::consts::TAU;

use glium::implement_vertex;
use nalgebra_glm::{mat2, vec2, Vec2};

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