use glium::implement_vertex;

#[derive(Clone, Copy, Debug)]
pub struct Vertex {
    pub position: [f32; 2],
    pub color: [f32; 4],
}
implement_vertex!(Vertex, position, color);
