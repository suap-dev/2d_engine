mod entity;
mod shaders;

use std::time::Duration;

use glium::{
    glutin::{dpi::PhysicalSize, event_loop::EventLoop, window::WindowBuilder, ContextBuilder},
    index::PrimitiveType,
    uniform, Display, DrawParameters, IndexBuffer, Program, Surface, VertexBuffer,
};
use nalgebra_glm::{rotation2d, vec2, vec2_to_vec3, vec3, Vec2};

use self::entity::Entity;
use super::{
    shape::{self, Shape},
    Vertex,
};

#[derive(Clone, Copy)]
pub struct EntityId(usize);

const WORLD_DIMENSIONS: [u32; 2] = [1000, 1000];
const GRAVITY: Vec2 = Vec2::new(0.0, -2.0);
const RADIUS: f32 = 0.01;
pub struct World {
    pub display: Display,
    entities: Vec<Entity>,
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

        #[allow(clippy::cast_precision_loss)]
        Self {
            display,
            program,
            sky_color: [0.0, 0.0, 0.0, 1.0],
            entities: Vec::with_capacity(4096),
            width: WORLD_DIMENSIONS[0] as f32,
            height: WORLD_DIMENSIONS[1] as f32,
            gravity: GRAVITY,
            default_shape: Shape::circle(RADIUS, [1.0, 1.0, 1.0, 1.0]),
            vertex_buffer: None,
            index_buffer: None,
        }
    }

    pub fn update_positions(&mut self, dt: f32) {
        for entity in &mut self.entities {
            entity.set_acceleration(self.gravity);
            entity.update_position(dt);
        }
        // self.solve_collisions();

        // self.update_vertex_buffer();
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

    pub fn to_gl_coords(&self, physical_coords: Vec2) -> Vec2 {
        let x = (physical_coords.x / self.width).mul_add(2.0, -1.0);
        let y = (physical_coords.y / self.height).mul_add(2.0, -1.0);

        vec2(x, -y)
    }

    pub fn entities_number(&self) -> usize {
        self.entities.len()
    }

    pub fn add_obj_at(&mut self, position: Vec2) {
        let new_entity = Entity::new(position, RADIUS, self.default_shape.color);

        self.entities.push(new_entity);

        self.rewrite_vertex_buffer();
        self.rewrite_index_buffer();
    }

    #[allow(clippy::cast_precision_loss)]
    pub fn populate(&mut self, columns: usize, rows: usize, origin: Vec2, rotation: f32) {
        let gap = RADIUS * 2.1;
        let x = -(columns as f32 / 2.0) * gap;
        let mut y = (rows as f32 / 2.0) * gap;

        for _row in 0..rows {
            for col in 0..columns {
                let temp_x = gap.mul_add(col as f32, x);
                let position = rotation2d(rotation) * vec3(temp_x, y, 1.0) + vec2_to_vec3(&origin);
                self.entities
                    .push(Entity::new(position.xy(), RADIUS, self.default_shape.color));
            }
            y -= gap;
        }
        self.rewrite_vertex_buffer();
        self.rewrite_index_buffer();
    }

    fn rewrite_vertex_buffer(&mut self) {
        let mut vertices: Vec<Vertex> = Vec::new();
        for entity in &self.entities {
            for vertex_position in &self.default_shape.vertices {
                let translated_vertex_position = vertex_position + entity.get_position();
                vertices.push(Vertex {
                    position: translated_vertex_position.into(),
                    color: entity.get_color(),
                });
            }
        }
        self.vertex_buffer = Some(
            VertexBuffer::new(&self.display, &vertices)
                .expect("Function rewrite_vertex_buffer() failed to create buffer."),
        );
    }

    fn rewrite_index_buffer(&mut self) {
        let mut indices: Vec<u16> = Vec::new();
        let entities = self.entities.len();

        #[allow(clippy::cast_possible_truncation)]
        for entity_nr in 0..entities {
            indices.extend_from_slice(
                &shape::CIRCLE_INDICES
                    .as_slice()
                    .iter()
                    .map(|v_idx| v_idx + entity_nr as u16 * shape::VERTICES_OF_A_CIRCLE)
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

    pub fn update_vertex_buffer(&mut self) {
        let mut vertices: Vec<Vertex> = Vec::new();
        for entity in &self.entities {
            for vertex_position in &self.default_shape.vertices {
                let translated_vertex_position = vertex_position + entity.get_position();
                vertices.push(Vertex {
                    position: translated_vertex_position.into(),
                    color: entity.get_color(),
                });
            }
        }
        if let Some(vertex_buffer) = &self.vertex_buffer {
            vertex_buffer.write(&vertices);
        }
    }

    // I'm not sure if this is going to be useful in any forseeable future
    // TODO: delete?
    #[allow(dead_code, clippy::cast_possible_truncation)]
    fn update_index_buffer(&mut self) {
        let mut indices: Vec<u16> = Vec::new();
        let entities = self.entities.len();
        for entity_nr in 0..entities {
            for index in shape::CIRCLE_INDICES {
                indices.push(index + entity_nr as u16 * shape::VERTICES_OF_A_CIRCLE);
            }
        }

        if let Some(index_buffer) = &self.index_buffer {
            index_buffer.write(indices.as_slice());
        }
    }

    pub fn solve_collisions(&mut self) {
        for i in 0..self.entities.len() {
            for j in i + 1..self.entities.len() {
                if self.entities[i].collides_with(&self.entities[j]) {
                    self.solve_collision(i, j);
                }
            }
        }
    }

    fn solve_collision(&mut self, entity1_idx: usize, entity2_idx: usize) {
        let delta_vector =
            self.entities[entity1_idx].get_position() - self.entities[entity2_idx].get_position();
        let distance = delta_vector.norm();
        let delta_vector = delta_vector.normalize() * (RADIUS - distance / 2.0);
        self.entities[entity1_idx].shift(delta_vector);
        self.entities[entity2_idx].shift(-delta_vector);
    }
}
