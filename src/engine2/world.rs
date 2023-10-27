use glium::{
    glutin::{dpi::PhysicalSize, event_loop::EventLoop, window::WindowBuilder, ContextBuilder},
    index::PrimitiveType,
    uniform, Display, DrawParameters, IndexBuffer, Program, Surface, VertexBuffer,
};
use grid::Grid;
use nalgebra_glm::{rotation2d, vec2, vec2_to_vec3, vec3, Vec2};

use crate::engine2::{
    entity::Entity,
    shaders,
    shape::{self, Shape},
    Vertex,
};

const WORLD_DIMENSIONS: [u32; 2] = [1000, 1000];
const GRAVITY: Vec2 = Vec2::new(0.0, -2.0);
const RADIUS: f32 = 0.005;

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
    // grid: Grid,
    grid: Grid<Vec<usize>>,
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

        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let grid_dimensions = (2.0 / RADIUS) as usize + 1;

        #[allow(clippy::cast_precision_loss)]
        Self {
            display,
            program,
            sky_color: [0.0, 0.0, 0.0, 1.0],
            entities: Vec::with_capacity(16_384),
            width: WORLD_DIMENSIONS[0] as f32,
            height: WORLD_DIMENSIONS[1] as f32,
            gravity: GRAVITY,
            default_shape: Shape::circle(RADIUS, [1.0, 1.0, 1.0, 1.0]),
            vertex_buffer: None,
            index_buffer: None,
            grid: Grid::new(grid_dimensions, grid_dimensions),
        }
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

    // TODO: is this the best way? it feels like a brute force.
    fn apply_constraint(entity: &mut Entity, constraint: &Constraint) {
        match constraint {
            Constraint::Circular => {
                const CONSTRAINT_CENTER: Vec2 = Vec2::new(0.0, 0.0);
                const CONSTRAINT_RADIUS: f32 = 0.9;

                let delta_vector = entity.position - CONSTRAINT_CENTER;

                if delta_vector.norm() > CONSTRAINT_RADIUS {
                    entity.position =
                        CONSTRAINT_CENTER + CONSTRAINT_RADIUS * delta_vector.normalize();
                }
            }
            Constraint::Rectangular => {
                const CONSTRAINT_BOUND: f32 = 0.9;

                entity.position.x = entity.position.x.clamp(-CONSTRAINT_BOUND, CONSTRAINT_BOUND);
                entity.position.y = entity.position.y.clamp(-CONSTRAINT_BOUND, CONSTRAINT_BOUND);
            }
        }
    }

    pub fn update_positions(&mut self, dt: f32) {
        for entity in &mut self.entities {
            entity.acceleration += self.gravity;

            // TODO: determine the correct order of these two
            entity.update_position(dt);
            Self::apply_constraint(entity, &Constraint::Rectangular);
        }

        // self.solve_collisions();
        // self.solve_collisions_with_grid();
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

    pub fn add_obj_at(&mut self, position: Vec2) {
        let new_entity = Entity::new(position, RADIUS, self.default_shape.color);

        self.entities.push(new_entity);

        self.rewrite_vertex_buffer();
        self.rewrite_index_buffer();
    }

    fn rewrite_vertex_buffer(&mut self) {
        let mut vertices: Vec<Vertex> = Vec::new();
        for entity in &self.entities {
            for vertex_position in &self.default_shape.vertices {
                let translated_vertex_position = vertex_position + entity.position;
                vertices.push(Vertex {
                    position: translated_vertex_position.into(),
                    color: entity.color,
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
        for entity_nr in 0..entities {
            #[allow(clippy::cast_possible_truncation)]
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
                let translated_vertex_position = vertex_position + entity.position;
                vertices.push(Vertex {
                    position: translated_vertex_position.into(),
                    color: entity.color,
                });
            }
        }
        if let Some(vertex_buffer) = &self.vertex_buffer {
            vertex_buffer.write(&vertices);
        }
    }

    // I'm not sure if this is going to be useful in any forseeable future
    fn update_index_buffer(&mut self) {
        let mut indices: Vec<u16> = Vec::new();
        let entities = self.entities.len();
        for entity_nr in 0..entities {
            for index in shape::CIRCLE_INDICES {
                #[allow(clippy::cast_possible_truncation)]
                indices.push(index + entity_nr as u16 * shape::VERTICES_OF_A_CIRCLE);
            }
        }

        if let Some(index_buffer) = &self.index_buffer {
            index_buffer.write(indices.as_slice());
        }
    }

    pub fn to_gl_coords(&self, physical_coords: Vec2) -> Vec2 {
        let x = (physical_coords.x / self.width).mul_add(2.0, -1.0);
        let y = (physical_coords.y / self.height).mul_add(2.0, -1.0);

        vec2(x, -y)
    }

    pub fn entities_number(&self) -> usize {
        self.entities.len()
    }

    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    fn get_ij(x: f32, y: f32) -> (usize, usize) {
        (((x + 1.0) / RADIUS) as usize, ((y + 1.0) / RADIUS) as usize)
    }

    pub fn solve_collisions_with_grid(&mut self) {
        self.grid.iter_mut().for_each(Vec::clear);
        for (idx, entity) in self.entities.iter().enumerate() {
            let (x, y) = Self::get_ij(entity.position.x, entity.position.y);
            self.grid[y][x].push(idx);
        }

        for r in 1..self.grid.rows() - 1 {
            for c in 1..self.grid.cols() - 1 {
                if !self.grid[r][c].is_empty() {
                    let mut possible_collisions: Vec<usize> = Vec::new();
                    for lr in r - 1..=r + 1 {
                        for lc in c - 1..=c + 1 {
                            let mut collisions_pocket = self.grid[lr][lc].clone();
                            possible_collisions.append(&mut collisions_pocket);
                        }
                    }
                    for (pos_1, &entity_index_1) in possible_collisions.iter().enumerate() {
                        for &entity_index_2 in possible_collisions.iter().skip(pos_1 + 1) {
                            let entity_1 = &self.entities[entity_index_1];
                            let entity_2 = &self.entities[entity_index_2];

                            if entity_1.collides_with(entity_2) {
                                self.solve_collision(entity_index_1, entity_index_2);
                            }
                        }
                    }
                }
            }
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
            self.entities[entity1_idx].position - self.entities[entity2_idx].position;
        let distance = delta_vector.norm();
        let delta_vector = delta_vector.normalize();
        self.entities[entity1_idx].position += delta_vector * (RADIUS - distance / 2.0);
        self.entities[entity2_idx].position -= delta_vector * (RADIUS - distance / 2.0);
    }
}

#[allow(dead_code)]
enum Constraint {
    Circular,
    Rectangular,
}
