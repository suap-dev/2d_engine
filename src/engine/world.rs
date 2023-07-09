use std::{collections::HashMap, time::Duration};

use glium::{
    glutin::{dpi::PhysicalSize, event_loop::EventLoop, window::WindowBuilder, ContextBuilder},
    uniform, Display, DrawParameters, Program, Surface, VertexBuffer,
};
use nalgebra_glm::{vec2, Vec2};

use super::{shape::Shape, Vertex};

// TODO: are separate structs Citizen and Entity really needed? Figure out and posibly make 1 struct for this functionality
// TODO: do I really want to keep the whole Entity in the World? do I need to remember it?
struct Citizen {
    shape: Shape,
    position: Vec2,
    velocity: Vec2,
    vertex_buffer: VertexBuffer<Vertex>,
}

#[derive(Clone, Copy)]
pub struct CitizenId(usize);

const WORLD_DIMENSIONS: [u32; 2] = [900, 900];
const ASPECT_RATIO: f32 = WORLD_DIMENSIONS[0] as f32 / WORLD_DIMENSIONS[1] as f32;
pub struct World {
    pub display: Display,
    citizens: HashMap<usize, Citizen>,
    sky_color: [f32; 4],
    program: Program,
    hash: usize,
    width: f32,
    height: f32,
    gravity: Vec2,
}

impl World {
    pub fn new<T>(event_loop: &EventLoop<T>) -> Self {
        let window_builder = WindowBuilder::new()
            .with_inner_size(PhysicalSize::new(WORLD_DIMENSIONS[0], WORLD_DIMENSIONS[1]))
            .with_resizable(false);
        let context_builder = ContextBuilder::new();
        let display = Display::new(window_builder, context_builder, event_loop)
            .expect("Unable to initialise display.");

        let program = Program::from_source(
            &display,
            r#"
            #version 150

            in vec2 position;

            uniform vec2 u_shape_origin;


            void main() {
                gl_Position = vec4(position+u_shape_origin, 0.0, 1.0);
            }
        "#,
            r#"
            #version 150

            uniform vec4 u_color;

            out vec4 color;

            void main() {
                color = u_color;
            }
        "#,
            None,
        )
        .expect("Program creation error.");
        Self {
            display,
            program,
            sky_color: [0.0, 0.0, 0.0, 1.0],
            citizens: HashMap::new(),
            hash: 0,
            width: WORLD_DIMENSIONS[0] as f32,
            height: WORLD_DIMENSIONS[1] as f32,
            gravity: vec2(0.0, -0.001),
        }
    }
    pub fn update(&mut self, dt: Duration) {
        for citizen in self.citizens.values_mut() {
            citizen.velocity += self.gravity * dt.as_secs_f32();            
            citizen.position += citizen.velocity;
        }
    }
    pub fn render(&self) {
        let mut frame = self.display.draw();
        frame.clear_color(
            self.sky_color[0],
            self.sky_color[1],
            self.sky_color[2],
            self.sky_color[3],
        );
        for citizen in self.citizens.values() {
            frame
                .draw(
                    &citizen.vertex_buffer,
                    glium::index::NoIndices(glium::index::PrimitiveType::TriangleFan),
                    &self.program,
                    &uniform! {
                        u_color: citizen.shape.color,
                        u_shape_origin: [citizen.position.x, citizen.position.y],
                    },
                    &DrawParameters::default(),
                )
                .expect("Unable to draw this entity.");
        }
        frame.finish().expect("Unable to finish drawing a frame.");
    }
    pub fn add(&mut self, shape: Shape, position: Vec2) -> CitizenId {
        self.hash += 1;
        self.citizens.insert(
            self.hash,
            Citizen {
                vertex_buffer: self.vertex_buffer(&shape),
                shape,
                position,
                velocity: vec2(0.0, 0.0),
            },
        );
        CitizenId(self.hash)
    }
    fn vertex_buffer(&self, shape: &Shape) -> VertexBuffer<Vertex> {
        let mut data = Vec::new();
        for vertex in &shape.vertices {
            data.push(Vertex {
                position: (*vertex).into(), // TODO: Transformation matrix needed. from logical to opengl? or should we do it on graphics card?
            });
        }
        VertexBuffer::new(&self.display, &data).expect("VertexBuffer creation failed.")
    }

    pub fn to_gl_coords(&self, physical_coords: Vec2) -> Vec2 {
        let x = (physical_coords.x as f32 / self.width) * 2.0 - 1.0;
        let y: f32 = (physical_coords.y as f32 / self.height) * 2.0 - 1.0;

        let v = vec2(x, -y);
        println!("{:?}", v);
        v
    }
}
