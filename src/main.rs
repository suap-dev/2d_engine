#![warn(clippy::all, clippy::nursery, clippy::pedantic)]

fn main() {
    let event_loop = EventLoop::new();
    let mut world = World::new(&event_loop);

    let mut triangle = Entity::empty();
    triangle.add_vertex(Vec2::new(0.0, 0.5)).unwrap();
    triangle.add_vertex(Vec2::new(-0.5, -0.5)).unwrap();
    triangle.add_vertex(Vec2::new(0.5, -0.5)).unwrap();

    world.add(triangle);

    event_loop.run(move |event, _, control_flow| {
        world.render();
        if let event::Event::WindowEvent { event, .. } = event {
            if let event::WindowEvent::CloseRequested = event {
                *control_flow = ControlFlow::Exit;
            }
        }
    });
}

use glium::{
    glutin::{
        event,
        event_loop::{ControlFlow, EventLoop},
        window::WindowBuilder,
        ContextBuilder,
    },
    implement_vertex,
    uniforms::EmptyUniforms,
    Display, DrawParameters, Program, Surface, VertexBuffer,
};

struct Citizen {
    vertex_buffer: VertexBuffer<Vertex>,
    program: Program,
    entity: Entity,
}

use nalgebra_glm::Vec2;
struct World {
    display: Display,
    citizens: Vec<Citizen>,
    sky_color: (f32, f32, f32),
}
impl World {
    fn new<T>(event_loop: &EventLoop<T>) -> Self {
        let window_builder = WindowBuilder::new();
        let context_builder = ContextBuilder::new();
        Self {
            display: Display::new(window_builder, context_builder, &event_loop)
                .expect("Unable to initialise display."),
            sky_color: (0.0, 0.0, 0.0),
            citizens: Vec::new(),
        }
    }
    fn render(&self) {
        let mut frame = self.display.draw();
        frame.clear_color(self.sky_color.0, self.sky_color.1, self.sky_color.2, 1.0);
        for citizen in &self.citizens {
            frame
                .draw(
                    &citizen.vertex_buffer,
                    glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList),
                    &citizen.program,
                    &EmptyUniforms,
                    &DrawParameters::default(),
                )
                .expect("Unable to draw {inhabitant:?}");
        }
        // frame.draw(_, _, program, uniforms, draw_parameters)
        frame.finish().expect("Unable to finish drawing a frame.");
    }
    fn add(&mut self, e: Entity) {
        self.citizens.push(Citizen {
            vertex_buffer: e.vertex_buffer(&self.display),
            program: Program::from_source(
                &self.display,
                r#"
                #version 150

                in vec2 position;

                void main() {
                    gl_Position = vec4(position, 0.0, 1.0);
                }
            "#,
                r#"
                #version 150

                out vec4 color;

                void main() {
                    color = vec4(1.0, 1.0, 1.0, 1.0);
                }
            "#,
                None,
            )
            .expect("Program creation error."),
            entity: e,
        });
    }
}

struct Entity {
    vertices: Vec<Vec2>,
    color: (f32, f32, f32),
    primitive: Primitive,
}
impl Entity {
    const fn empty() -> Self {
        Self {
            vertices: Vec::new(),
            color: (1.0, 1.0, 1.0),
            primitive: Primitive::Empty,
        }
    }

    fn add_vertex(&mut self, v: Vec2) -> Result<Primitive, Primitive> {
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
        let thingy: Vec<Vertex> = self.vertices.iter().map(|v| (*v).into()).collect();
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
enum Primitive {
    Empty,
    Point,
    Line,
    Polygon,
    Circle,
}

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
