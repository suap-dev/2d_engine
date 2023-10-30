use std::f32::consts::PI;

use glium::glutin::event_loop::EventLoop;
use grid::Grid;
use itertools::Itertools;
use nalgebra_glm::{rotation2d, vec2, vec2_to_vec3, vec3, Vec2};

use crate::engine2::{
    graphics::Renderer, objects_generator::ObjectsGenerator, verlet_object::VerletObject,
};

const WORLD_HEIGHT: u16 = 1000;
const WORLD_WIDTH: u16 = 1000;

const GRID_ROWS: u16 = 50;
const GRID_COLS: u16 = 50;

const ROW_HEIGHT: f32 = 2.0 / GRID_ROWS as f32;
const COL_WIDTH: f32 = 2.0 / GRID_COLS as f32;

const GRAVITY: Vec2 = Vec2::new(0.0, -1.0);

pub struct World {
    objects: Vec<VerletObject>,
    gravity: Vec2,
    renderer: Renderer,
}
impl World {
    pub fn new<T>(event_loop: &EventLoop<T>) -> Self {
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        #[allow(clippy::cast_precision_loss)]
        Self {
            objects: Vec::new(),
            gravity: GRAVITY,
            renderer: Renderer::new(event_loop, WORLD_WIDTH as u32, WORLD_HEIGHT as u32),
        }
    }

    pub fn update(&mut self, dt: f32, substeps: usize) {
        let dt = dt / substeps as f32;
        for _ in 0..substeps {
            self.apply_gravity();

            // TODO: determine the correct order of these two
            self.constrain(Constraint::Rectangular);

            // self.solve_collisions();
            self.solve_collisions_with_grid();

            self.update_positions(dt);
        }
    }

    #[allow(clippy::cast_precision_loss)]
    pub fn populate(&mut self, generator: &mut ObjectsGenerator) {
        let distance = (generator.obj_radius + generator.obj_radius_deviation)
            .mul_add(2.0, generator.obj_min_separation);
        let x = -(generator.grid_columns as f32 / 2.0) * distance;
        let mut y = (generator.grid_rows as f32 / 2.0) * distance;

        for _row in 0..generator.grid_rows {
            for col in 0..generator.grid_columns {
                let temp_x = distance.mul_add(col as f32, x);
                let center = rotation2d(generator.grid_rotation) * vec3(temp_x, y, 1.0)
                    + vec2_to_vec3(&generator.grid_center);

                let radius = generator.random_radius();

                self.objects
                    .push(VerletObject::new(center.xy(), radius, [1.0, 1.0, 1.0, 1.0]));
            }
            y -= distance;
        }
        self.renderer.rewrite_vertex_buffer(&self.objects);
        self.renderer.rewrite_index_buffer(&self.objects);
    }

    fn constrain(&mut self, constraint: Constraint) {
        self.objects.iter_mut().for_each(|obj| {
            if let Some(offset) = Self::trespass_vector(obj, constraint) {
                // obj.adjust_position_data(-offset);
                obj.shift(-offset);
            }
        });
    }

    // TODO: is this the best way? it feels like a brute force.
    fn trespass_vector(obj: &VerletObject, constraint: Constraint) -> Option<Vec2> {
        match constraint {
            Constraint::Circular => {
                const CONSTRAINT_CENTER: Vec2 = Vec2::new(0.0, 0.0);
                const CONSTRAINT_RADIUS: f32 = 0.9;

                let distance_from_center = obj.get_center().metric_distance(&CONSTRAINT_CENTER);
                if distance_from_center + obj.get_radius() > CONSTRAINT_RADIUS {
                    let distance_vec = obj.get_center() - CONSTRAINT_CENTER;
                    let radius_vec =
                        distance_vec.normalize() * (CONSTRAINT_RADIUS - obj.get_radius());
                    let trespass_vec = distance_vec - radius_vec;
                    Some(trespass_vec)
                } else {
                    None
                }
            }
            Constraint::Rectangular => {
                const CONSTRAINT_BOUND: f32 = 0.9;

                let max_center = CONSTRAINT_BOUND - obj.get_radius();

                let clamped = vec2(
                    obj.get_center().x.clamp(-max_center, max_center),
                    obj.get_center().y.clamp(-max_center, max_center),
                );

                if clamped == obj.get_center() {
                    None
                } else {
                    Some(obj.get_center() - clamped)
                }
            }
        }
    }

    fn apply_gravity(&mut self) {
        self.objects
            .iter_mut()
            .for_each(|obj| obj.set_acceleration(self.gravity));
    }

    pub fn update_positions(&mut self, dt: f32) {
        self.objects.iter_mut().for_each(|obj| {
            obj.update_position(dt);
        });
    }

    pub fn add_obj_at(&mut self, center: Vec2, radius: f32) {
        let new_obj = VerletObject::new(center, radius, [1.0, 1.0, 1.0, 1.0]);

        self.objects.push(new_obj);

        self.renderer.rewrite_vertex_buffer(&self.objects);
        self.renderer.rewrite_index_buffer(&self.objects);
    }

    pub fn to_gl_coords(&self, physical_coords: Vec2) -> Vec2 {
        let x = (physical_coords.x / WORLD_WIDTH as f32).mul_add(2.0, -1.0);
        let y = (physical_coords.y / WORLD_HEIGHT as f32).mul_add(2.0, -1.0);

        vec2(x, -y)
    }

    pub fn objects_number(&self) -> usize {
        self.objects.len()
    }

    pub fn solve_collisions_with_grid(&mut self) {
        let mut grid: Grid<Vec<usize>> = Grid::new(GRID_ROWS as usize, GRID_COLS as usize);

        for (idx, obj) in self.objects.iter().enumerate() {
            let mut i = obj.get_center().x;
            let mut j = obj.get_center().y;

            i += 1.0;
            let i = (i / COL_WIDTH).trunc() as usize;

            j += 1.0;
            let j = (j / ROW_HEIGHT).trunc() as usize;

            grid[i][j].push(idx);
        }

        for row in 1..grid.rows() - 1 {
            for col in 1..grid.cols() - 1 {
                let start_row = row - 1;
                let end_row = row + 1;

                let start_col = col - 1;
                let end_col = col + 1;

                let mut big_pocket = Vec::new();

                for row in start_row..end_row {
                    for col in start_col..end_col {
                        big_pocket.append(&mut grid[row][col].clone());
                    }
                }

                big_pocket.iter().tuple_combinations().for_each(|(&i, &j)| {
                    if self.objects[i].collides_with(&self.objects[j]) {
                        self.solve_collision(i, j);
                    }
                });
            }
        }
    }

    pub fn solve_collisions(&mut self) {
        (0..self.objects.len())
            .tuple_combinations()
            .for_each(|(i, j)| {
                if self.objects[i].collides_with(&self.objects[j]) {
                    self.solve_collision(i, j);
                }
            });
    }

    fn solve_collision(&mut self, obj1_idx: usize, obj2_idx: usize) {
        let obj1 = &self.objects[obj1_idx];
        let obj2 = &self.objects[obj2_idx];

        let centers_distance = obj2.get_center().metric_distance(&obj1.get_center());
        let radius_sum = obj2.get_radius() + obj1.get_radius();

        if centers_distance < radius_sum {
            let delta_versor = (obj2.get_center() - obj1.get_center()).normalize();
            let m1 = PI * obj1.get_radius().powi(2);
            let m2 = PI * obj2.get_radius().powi(2);

            let adjustment_vector = delta_versor * (radius_sum - centers_distance);

            let adjustment1 = -(m2 / (m1 + m2)) * adjustment_vector;
            let adjustment2 = (m1 / (m1 + m2)) * adjustment_vector;

            // let mut obj1 = &obj1;
            self.objects[obj1_idx].shift(adjustment1);
            self.objects[obj2_idx].shift(adjustment2);
        }
    }

    pub fn update_vertex_buffer(&mut self) {
        self.renderer.update_vertex_buffer(&self.objects);
    }

    pub fn render(&self) {
        self.renderer.render();
    }
}

#[derive(Clone, Copy)]
#[allow(dead_code)]
enum Constraint {
    Circular,
    Rectangular,
}
