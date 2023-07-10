mod shaders;

use super::{
    shape::{self, Shape},
    Vertex,
};
use glium::{
    glutin::{dpi::PhysicalSize, event_loop::EventLoop, window::WindowBuilder, ContextBuilder},
    index::PrimitiveType,
    uniform, Display, DrawParameters, IndexBuffer, Program, Surface, VertexBuffer,
};
use nalgebra_glm::{rotation2d, vec2, vec2_to_vec3, vec3, Vec2};
use std::time::Duration;

struct Citizen {
    position: Vec2,
    previous_position: Vec2,
    acceleration: Vec2,
    velocity: Vec2,
    color: [f32; 4],
}

impl Citizen {
    fn update_position(&mut self, dt: Duration) {
        let delta_position = self.position - self.previous_position;
        let dt = dt.as_secs_f32();
        self.previous_position = self.position;

        self.position = self.position + delta_position + self.acceleration * dt * dt;
        self.acceleration.fill(0.0);

        self.apply_constraints(1);
    }
    fn apply_constraints(&mut self, constraint: u16) {
        match constraint {
            0 => {
                const CONSTRAINT_CENTER: Vec2 = Vec2::new(0.0, 0.0);
                const CONSTRAINT_RADIUS: f32 = 0.9;

                let delta_vector = self.position - CONSTRAINT_CENTER;

                if delta_vector.norm() > CONSTRAINT_RADIUS {
                    self.position =
                        CONSTRAINT_CENTER + CONSTRAINT_RADIUS * delta_vector.normalize();
                }
            }
            1 => {
                const CONSTRAINT_BOUND: f32 = 0.9;

                if self.position.x > CONSTRAINT_BOUND {
                    self.position.x = CONSTRAINT_BOUND;
                } else if self.position.x < -CONSTRAINT_BOUND {
                    self.position.x = -CONSTRAINT_BOUND;
                }

                if self.position.y > CONSTRAINT_BOUND {
                    self.position.y = CONSTRAINT_BOUND;
                } else if self.position.y < -CONSTRAINT_BOUND {
                    self.position.y = -CONSTRAINT_BOUND;
                }
            }
            _ => {}
        }
    }

    fn collides_with(&self, other: &Self) -> bool {
        self.position.metric_distance(&other.position) < 2.0 * RADIUS
    }

    fn new_at(position: Vec2) -> Self {
        Self {
            position,
            previous_position: position,
            acceleration: Vec2::zeros(),
            velocity: Vec2::zeros(),
            color: [1.0, 1.0, 0.0, 1.0],
        }
    }
}

#[derive(Clone, Copy)]
pub struct CitizenId(usize);

const WORLD_DIMENSIONS: [u32; 2] = [1000, 1000];
const GRAVITY: Vec2 = Vec2::new(0.0, -2.0);
const RADIUS: f32 = 0.01;
pub struct World {
    pub display: Display,
    citizens: Vec<Citizen>,
    sky_color: [f32; 4],
    program: Program,
    width: f32,
    height: f32,
    gravity: Vec2,
    default_shape: Shape,
    vertex_buffer: Option<VertexBuffer<Vertex>>,
    index_buffer: Option<IndexBuffer<u16>>,
}

impl World {
    pub fn new<T>(event_loop: &EventLoop<T>) -> Self {
        let window_builder = WindowBuilder::new()
            .with_inner_size(PhysicalSize::new(WORLD_DIMENSIONS[0], WORLD_DIMENSIONS[1]))
            .with_resizable(false);
        let context_builder = ContextBuilder::new();
        let display = Display::new(window_builder, context_builder, event_loop)
            .expect("Unable to initialise display.");

        let program = Program::from_source(&display, shaders::VERTEX, shaders::FRAGMENT, None)
            .expect("Program creation error.");
        Self {
            display,
            program,
            sky_color: [0.0, 0.0, 0.0, 1.0],
            citizens: Vec::with_capacity(4096),
            width: WORLD_DIMENSIONS[0] as f32,
            height: WORLD_DIMENSIONS[1] as f32,
            gravity: GRAVITY,
            default_shape: Shape::circle(RADIUS, [1.0, 0.0, 0.0, 1.1]),
            vertex_buffer: None,
            index_buffer: None,
        }
    }
    pub fn fill(&mut self, columns: usize, rows: usize, origin: Vec2, rotation: f32) {
        let gap = RADIUS * 2.1;
        let mut x = -(columns as f32 / 2.0) * gap;
        let mut y = (rows as f32 / 2.0) * gap;

        for row in 0..rows {
            for col in 0..columns {
                let temp_x = x + gap * (col as f32);
                let position = rotation2d(rotation) * vec3(temp_x, y, 1.0) + vec2_to_vec3(&origin);
                self.citizens.push(Citizen::new_at(position.xy()));
            }
            y -= gap;
        }
        self.rewrite_vertex_buffer();
        self.rewrite_index_buffer();
    }
    pub fn update(&mut self, dt: Duration) {
        for citizen in &mut self.citizens {
            citizen.acceleration += self.gravity;
            citizen.update_position(dt);
        }

        self.solve_collisions();

        self.update_vertex_buffer();
    }
    pub fn render(&self) {
        let mut frame = self.display.draw();
        frame.clear_color(
            self.sky_color[0],
            self.sky_color[1],
            self.sky_color[2],
            self.sky_color[3],
        );
        if let Some(vb) = &self.vertex_buffer {
            if let Some(ib) = &self.index_buffer {
                frame
                    .draw(
                        vb,
                        ib,
                        &self.program,
                        &uniform! {
                            u_color: [1.0f32, 0.0, 1.0, 1.0],
                        },
                        &DrawParameters::default(),
                    )
                    .expect("Unable to draw this entity.");
            };
        }
        frame.finish().expect("Unable to finish drawing a frame.");
    }
    pub fn add_obj_at(&mut self, position: Vec2) {
        let new_citizen = Citizen {
            position,
            velocity: vec2(0.0, 0.0),
            color: self.default_shape.color,
            acceleration: vec2(0.0, 0.0),
            previous_position: position,
        };
        self.citizens.push(new_citizen);

        self.rewrite_vertex_buffer();
        self.rewrite_index_buffer();
    }
    fn rewrite_vertex_buffer(&mut self) {
        let mut vertices: Vec<Vertex> = Vec::new();
        for citizen in &self.citizens {
            for vertex in &self.default_shape.vertices {
                let vert = vertex + citizen.position;
                vertices.push(vert.into());
            }
        }
        self.vertex_buffer = Some(
            VertexBuffer::new(&self.display, &vertices)
                .expect("Function rewrite_vertex_buffer() failed to create buffer."),
        );
    }
    fn rewrite_index_buffer(&mut self) {
        let mut indices: Vec<u16> = Vec::new();
        let citizens = self.citizens.len();
        for citizen_nr in 0..citizens {
            indices.extend_from_slice(
                &shape::CIRCLE_INDICES
                    .as_slice()
                    .iter()
                    .map(|v_idx| v_idx + citizen_nr as u16 * shape::VERTICES_OF_A_CIRCLE)
                    .collect::<Vec<u16>>(),
            );
        }
        self.index_buffer = Some(
            IndexBuffer::new(
                &self.display,
                PrimitiveType::TrianglesList,
                indices.as_slice(),
            )
            .expect("Function rewrite_index_buffer() failed to create buffer."),
        );
    }
    fn update_vertex_buffer(&mut self) {
        let mut vertices: Vec<Vertex> = Vec::new();
        for citizen in &self.citizens {
            for vertex in &self.default_shape.vertices {
                let vert = vertex + citizen.position;
                vertices.push(vert.into());
            }
        }
        if let Some(vertex_buffer) = &self.vertex_buffer {
            vertex_buffer.write(&vertices);
        }
    }
    // I'm not sure if this is going to be useful in any forseeable future
    fn update_index_buffer(&mut self) {
        let mut indices: Vec<u16> = Vec::new();
        let citizens = self.citizens.len();
        for citizen_nr in 0..citizens {
            for index in shape::CIRCLE_INDICES {
                indices.push(index + citizen_nr as u16 * shape::VERTICES_OF_A_CIRCLE);
            }
        }

        if let Some(index_buffer) = &self.index_buffer {
            index_buffer.write(indices.as_slice());
        }
    }
    pub fn to_gl_coords(&self, physical_coords: Vec2) -> Vec2 {
        let x = (physical_coords.x as f32 / self.width) * 2.0 - 1.0;
        let y = (physical_coords.y as f32 / self.height) * 2.0 - 1.0;

        vec2(x, -y)
    }
    pub fn citizens_number(&self) -> usize {
        self.citizens.len()
    }
    fn solve_collisions(&mut self) {
        for i in 0..self.citizens.len() {
            for j in i + 1..self.citizens.len() {
                if self.citizens[i].collides_with(&self.citizens[j]) {
                    let delta_vector = self.citizens[i].position - self.citizens[j].position;
                    let distance = delta_vector.norm();
                    let delta_vector = delta_vector.normalize();
                    self.citizens[i].position += delta_vector * (RADIUS - distance / 2.0);
                    self.citizens[j].position -= delta_vector * (RADIUS - distance / 2.0);
                }
            }
        }
    }
}
