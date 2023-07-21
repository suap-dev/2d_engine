mod entity;
mod shaders;
mod shape;
pub mod world;


use glium::implement_vertex;

#[derive(Clone, Copy, Debug)]
struct Vertex {
    position: [f32; 2],
    color: [f32; 4],
}
implement_vertex!(Vertex, position, color);
