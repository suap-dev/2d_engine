use std::{collections::HashMap, time::Duration};

use glium::{
    glutin::{dpi::PhysicalSize, event_loop::EventLoop, window::WindowBuilder, ContextBuilder},
    index::{Index, PrimitiveType},
    uniform, Display, DrawParameters, IndexBuffer, Program, Surface, VertexBuffer,
};
use nalgebra_glm::{vec2, Vec2};

use super::{
    shape::{self, Shape},
    Vertex,
};

struct Citizen {
    position: Vec2,
    velocity: Vec2,
    color: [f32; 4],
}

#[derive(Clone, Copy)]
pub struct CitizenId(usize);

const WORLD_DIMENSIONS: [u32; 2] = [900, 900];
const GRAVITY: Vec2 = Vec2::new(0.0, 0.0);
const RADIUS: f32 = 0.01;
pub struct World {
    pub display: Display,
    citizens: HashMap<usize, Citizen>,
    sky_color: [f32; 4],
    program: Program,
    hash: usize,
    width: f32,
    height: f32,
    gravity: Vec2,
    default_shape: Shape,
    // TODO: We actualy need to make one vertex_buffer and one index_buffer
    default_shape_vertex_buffer: Option<VertexBuffer<Vertex>>,
    default_shape_index_buffer: Option<IndexBuffer<u16>>,
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
                gl_Position = vec4(position + u_shape_origin, 0.0, 1.0);
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
        let mut new_world = Self {
            display,
            program,
            sky_color: [0.0, 0.0, 0.0, 1.0],
            citizens: HashMap::new(),
            hash: 0,
            width: WORLD_DIMENSIONS[0] as f32,
            height: WORLD_DIMENSIONS[1] as f32,
            gravity: GRAVITY,
            default_shape: Shape::circle(RADIUS, [1.0, 0.0, 0.0, 1.1]),
            default_shape_vertex_buffer: None,
            default_shape_index_buffer: None,
        };
        new_world.default_shape_vertex_buffer =
            Some(new_world.vertex_buffer(&new_world.default_shape));
        new_world.default_shape_index_buffer = Some(new_world.index_buffer());

        new_world
    }
    pub fn update(&mut self, dt: Duration) {
        self.citizens.retain(|_, citizen| {
            citizen.velocity += self.gravity * dt.as_secs_f32();
            citizen.position += citizen.velocity;

            if citizen.velocity.y < -0.0015 {
                citizen.velocity.y = -0.0015
            }

            if citizen.position.y.abs() > 0.99 {
                citizen.position.y = 0.98
            };

            // which citizens to retain:
            // !(citizen.position.x.abs() > 1.0 || citizen.position.y.abs() > 1.0)
            true
        });
    }
    pub fn render(&self) {
        let mut frame = self.display.draw();
        frame.clear_color(
            self.sky_color[0],
            self.sky_color[1],
            self.sky_color[2],
            self.sky_color[3],
        );
        if let (Some(vb)) = &self.default_shape_vertex_buffer {
            if let (Some(ib)) = &self.default_shape_index_buffer {
                for citizen in self.citizens.values() {
                    frame
                        .draw(
                            vb,
                            ib,
                            &self.program,
                            &uniform! {
                                u_color: citizen.color,
                                u_shape_origin: [citizen.position.x, citizen.position.y],
                            },
                            &DrawParameters::default(),
                        )
                        .expect("Unable to draw this entity.");
                }
            };
        }
        frame.finish().expect("Unable to finish drawing a frame.");
    }
    pub fn add_default(&mut self, position: Vec2) -> CitizenId {
        self.hash += 1;
        self.citizens.insert(
            self.hash,
            Citizen {
                position,
                velocity: vec2(0.0, 0.0),
                color: self.default_shape.color, // TODO: add changing colors :)
            },
        );
        println!("{:?}/{:?}", &self.citizens.len(), &self.citizens.capacity());
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
    fn index_buffer(&self) -> IndexBuffer<u16> {
        IndexBuffer::new(
            &self.display,
            PrimitiveType::TrianglesList,
            &shape::CIRCLE_INDICES,
        )
        .unwrap()
    }

    pub fn to_gl_coords(&self, physical_coords: Vec2) -> Vec2 {
        let x = (physical_coords.x as f32 / self.width) * 2.0 - 1.0;
        let y: f32 = (physical_coords.y as f32 / self.height) * 2.0 - 1.0;

        let v = vec2(x, -y);
        v
    }
}
