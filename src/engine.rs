use glium::{VertexBuffer, Program, Display, glutin::{event_loop::EventLoop, window::WindowBuilder, ContextBuilder}, Surface, uniforms::EmptyUniforms, DrawParameters, implement_vertex};
use nalgebra_glm::Vec2;

struct Citizen {
    vertex_buffer: VertexBuffer<Vertex>,
    program: Program,
    entity: Entity,
}
pub struct World {
    display: Display,
    citizens: Vec<Citizen>,
    sky_color: (f32, f32, f32),
}
impl World {
    pub fn new<T>(event_loop: &EventLoop<T>) -> Self {
        let window_builder = WindowBuilder::new();
        let context_builder = ContextBuilder::new();
        Self {
            display: Display::new(window_builder, context_builder, event_loop)
                .expect("Unable to initialise display."),
            sky_color: (0.0, 0.0, 0.0),
            citizens: Vec::new(),
        }
    }
    pub fn render(&self) {
        let mut frame = self.display.draw();
        frame.clear_color(self.sky_color.0, self.sky_color.1, self.sky_color.2, 1.0);
        for citizen in &self.citizens {
            frame
                .draw(
                    &citizen.vertex_buffer,
                    glium::index::NoIndices(glium::index::PrimitiveType::TriangleFan),
                    &citizen.program,
                    &EmptyUniforms,
                    &DrawParameters::default(),
                )
                .expect("Unable to draw {inhabitant:?}");
        }
        // frame.draw(_, _, program, uniforms, draw_parameters)
        frame.finish().expect("Unable to finish drawing a frame.");
    }
    pub fn add(&mut self, e: Entity) {
        self.citizens.push(Citizen {
            vertex_buffer: e.vertex_buffer(&self.display),
            program: Program::from_source(
                &self.display,
                r#"
                #version 150

                in vec2 position;
                in vec4 color;

                out vec4 v_color;

                void main() {
                    v_color = color;
                    gl_Position = vec4(position, 0.0, 1.0);
                }
            "#,
                r#"
                #version 150

                in vec4 v_color;
                out vec4 color;

                void main() {
                    // color = vec4(1.0, 1.0, 1.0, 1.0);
                    color = v_color;
                }
            "#,
                None,
            )
            .expect("Program creation error."),
            entity: e,
        });
    }
}

pub struct Entity {
    vertices: Vec<Vec2>,
    color: [f32; 4],
    primitive: Primitive,
}
impl Entity {
    pub const fn empty() -> Self {
        Self {
            vertices: Vec::new(),
            color: [1.0, 1.0, 1.0, 1.0],
            primitive: Primitive::Empty,
        }
    }

    pub fn add_vertex(&mut self, v: Vec2) -> Result<Primitive, Primitive> {
        match self.primitive {
            Primitive::Circle => Err(Primitive::Circle),
            Primitive::Empty => {
                self.vertices.push(v);
                self.primitive = Primitive::Point;
                Ok(Primitive::Point)
            }
            Primitive::Point => {
                self.vertices.push(v);
                self.primitive = Primitive::Line;
                Ok(Primitive::Line)
            }
            Primitive::Line | Primitive::Polygon => {
                self.vertices.push(v);
                self.primitive = Primitive::Polygon;
                Ok(Primitive::Polygon)
            }
        }
    }

    fn vertex_buffer(&self, display: &Display) -> VertexBuffer<Vertex> {
        // let thingy: Vec<Vertex> = self.vertices.iter().map(|v| (*v).into()).collect();
        VertexBuffer::new(
            display,
            &self
                .vertices
                .iter()
                .map(|v| (*v).into())
                .collect::<Vec<Vertex>>(),
        )
        .expect("VertexBuffer creation failed.")
    }
}

#[derive(Debug)]
pub enum Primitive {
    Empty,
    Point,
    Line,
    Polygon,
    Circle,
}

#[derive(Clone, Copy)]
struct Vertex {
    position: [f32; 2],
    color: [f32; 4],
}
impl From<Vec2> for Vertex {
    fn from(value: Vec2) -> Self {
        Self {
            position: value.into(),
            // color: [1.0; 4]
            color: [1.0, 0.0, 0.0, 1.0],
        }
    }
}
implement_vertex!(Vertex, position, color);
