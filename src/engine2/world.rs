use std::f32::consts::PI;

use glium::glutin::event_loop::EventLoop;
use grid::Grid;
use nalgebra_glm::{rotation2d, vec2, vec2_to_vec3, vec3, Vec2};
use rand::random;

use crate::engine2::{graphics::Renderer, verlet_object::VerletObject};

const WORLD_DIMENSIONS: [u32; 2] = [1000, 1000];
const GRAVITY: Vec2 = Vec2::new(0.0, -0.1);

pub struct World {
    objects: Vec<VerletObject>,
    width: f32,
    height: f32,
    gravity: Vec2,
    // grid: Grid,
    // grid: Grid<Vec<usize>>,
    renderer: Renderer,
}
impl World {
    pub fn new<T>(event_loop: &EventLoop<T>) -> Self {
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        // let grid_dimensions = (2.0 / RADIUS) as usize + 1;
        #[allow(clippy::cast_precision_loss)]
        Self {
            objects: Vec::with_capacity(16_384),
            width: WORLD_DIMENSIONS[0] as f32,
            height: WORLD_DIMENSIONS[1] as f32,
            gravity: GRAVITY,
            // grid: Grid::new(grid_dimensions, grid_dimensions),
            renderer: Renderer::new(event_loop, WORLD_DIMENSIONS),
        }
    }

    #[allow(clippy::cast_precision_loss)]
    pub fn populate(
        &mut self,
        columns: usize,
        rows: usize,
        origin: Vec2,
        rotation: f32,
        radius: f32,
        radius_deviation: f32,
        deviation_seed: i32,
        min_distance: f32,
    ) {
        let distance = (radius + radius_deviation).mul_add(2.0, min_distance);
        let x = -(columns as f32 / 2.0) * distance;
        let mut y = (rows as f32 / 2.0) * distance;

        for _row in 0..rows {
            for col in 0..columns {
                let temp_x = distance.mul_add(col as f32, x);
                let center = rotation2d(rotation) * vec3(temp_x, y, 1.0) + vec2_to_vec3(&origin);

                let randomizer = random::<f32>().mul_add(2.0, -1.0);
                let delta_radius = randomizer * radius_deviation;
                let radius = radius + delta_radius;

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

    pub fn update(&mut self, dt: f32, substeps: usize) {
        let dt = dt / substeps as f32;
        for _ in 0..substeps {
            self.apply_gravity();

            // TODO: determine the correct order of these two
            self.constrain(Constraint::Rectangular);
            self.solve_collisions();

            // self.solve_collisions_with_grid();
            self.update_positions(dt);
            // self.update_posiion
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

        // self.solve_collisions();
        // self.solve_collisions_with_grid();
        // self.update_vertex_buffer();
    }

    pub fn add_obj_at(&mut self, center: Vec2, radius: f32) {
        let new_obj = VerletObject::new(center, radius, [1.0, 1.0, 1.0, 1.0]);

        self.objects.push(new_obj);

        self.renderer.rewrite_vertex_buffer(&self.objects);
        self.renderer.rewrite_index_buffer(&self.objects);
    }

    pub fn to_gl_coords(&self, physical_coords: Vec2) -> Vec2 {
        let x = (physical_coords.x / self.width).mul_add(2.0, -1.0);
        let y = (physical_coords.y / self.height).mul_add(2.0, -1.0);

        vec2(x, -y)
    }

    pub fn entities_number(&self) -> usize {
        self.objects.len()
    }

    // #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    // fn get_ij(x: f32, y: f32) -> (usize, usize) {
    //     (((x + 1.0) / RADIUS) as usize, ((y + 1.0) / RADIUS) as usize)
    // }

    // pub fn solve_collisions_with_grid(&mut self) {
    //     self.grid.iter_mut().for_each(Vec::clear);
    //     for (idx, obj) in self.objects.iter().enumerate() {
    //         let (x, y) = Self::get_ij(obj.get_center().x, obj.get_center().y);
    //         self.grid[y][x].push(idx);
    //     }

    //     for r in 1..self.grid.rows() - 1 {
    //         for c in 1..self.grid.cols() - 1 {
    //             if !self.grid[r][c].is_empty() {
    //                 let mut possible_collisions: Vec<usize> = Vec::new();
    //                 for lr in r - 1..=r + 1 {
    //                     for lc in c - 1..=c + 1 {
    //                         let mut collisions_pocket = self.grid[lr][lc].clone();
    //                         possible_collisions.append(&mut collisions_pocket);
    //                     }
    //                 }
    //                 for (pos_1, &obj_index_1) in possible_collisions.iter().enumerate() {
    //                     for &obj_index_2 in possible_collisions.iter().skip(pos_1 + 1) {
    //                         let obj_1 = &self.objects[obj_index_1];
    //                         let obj_2 = &self.objects[obj_index_2];

    //                         if obj_1.collides_with(obj_2) {
    //                             self.solve_collision(obj_index_1, obj_index_2);
    //                         }
    //                     }
    //                 }
    //             }
    //         }
    //     }
    // }

    pub fn solve_collisions(&mut self) {
        for i in 0..self.objects.len() {
            for j in i + 1..self.objects.len() {
                if self.objects[i].collides_with(&self.objects[j]) {
                    self.solve_collision(i, j);
                }
            }
        }
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

        // let delta_vector =
        //     self.objects[obj1_idx].get_position() - self.objects[obj2_idx].get_position();
        // let distance = delta_vector.norm();
        // let delta = delta_vector.normalize() * (RADIUS - distance / 2.0);
        // self.objects[obj1_idx].shift(delta);
        // self.objects[obj2_idx].shift(-delta);
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
