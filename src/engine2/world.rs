use glium::glutin::event_loop::EventLoop;
use grid::Grid;
use nalgebra_glm::{rotation2d, vec2, vec2_to_vec3, vec3, Vec2};

use crate::engine2::{
    graphics::{shape::RADIUS, Renderer},
    verlet_object::VerletObject,
};

const WORLD_DIMENSIONS: [u32; 2] = [1000, 1000];
const GRAVITY: Vec2 = Vec2::new(0.0, -2.0);

pub struct World {
    objects: Vec<VerletObject>,
    width: f32,
    height: f32,
    gravity: Vec2,
    // grid: Grid,
    grid: Grid<Vec<usize>>,
    renderer: Renderer,
}
impl World {
    pub fn new<T>(event_loop: &EventLoop<T>) -> Self {
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let grid_dimensions = (2.0 / RADIUS) as usize + 1;

        #[allow(clippy::cast_precision_loss)]
        Self {
            objects: Vec::with_capacity(16_384),
            width: WORLD_DIMENSIONS[0] as f32,
            height: WORLD_DIMENSIONS[1] as f32,
            gravity: GRAVITY,
            grid: Grid::new(grid_dimensions, grid_dimensions),
            renderer: Renderer::new(event_loop, WORLD_DIMENSIONS),
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
                self.objects.push(VerletObject::new(
                    position.xy(),
                    RADIUS,
                    self.renderer.default_shape.color,
                ));
            }
            y -= gap;
        }
        self.renderer.rewrite_vertex_buffer(&self.objects);
        self.renderer.rewrite_index_buffer(&self.objects);
    }

    fn apply_constraint(&mut self, constraint: &Constraint) {
        match constraint {
            Constraint::Circular => {
                const CONSTRAINT_CENTER: Vec2 = Vec2::new(0.0, 0.0);
                const CONSTRAINT_RADIUS: f32 = 0.9;

                self.objects.iter_mut().for_each(|obj| {
                    let relative_position = obj.get_position() - CONSTRAINT_CENTER;
                    let distance = relative_position.norm();
                    let out_of_bounds = distance + obj.get_radius() - CONSTRAINT_RADIUS;

                    if out_of_bounds > 0.0 {
                        let fix = -relative_position.normalize() * out_of_bounds;
                        obj.shift(fix);
                    }
                });
            }
            Constraint::Rectangular => {
                const CONSTRAINT_BOUND: f32 = 0.9;

                self.objects.iter_mut().for_each(|obj| {
                    obj.set_position(Vec2::new(
                        obj.get_position()
                            .x
                            .clamp(-CONSTRAINT_BOUND, CONSTRAINT_BOUND),
                        obj.get_position()
                            .y
                            .clamp(-CONSTRAINT_BOUND, CONSTRAINT_BOUND),
                    ));
                });
            }
        }
    }

    // TODO: is this the best way? it feels like a brute force.
    fn apply_constraint2(obj: &mut VerletObject, constraint: &Constraint) {
        match constraint {
            Constraint::Circular => {
                const CONSTRAINT_CENTER: Vec2 = Vec2::new(0.0, 0.0);
                const CONSTRAINT_RADIUS: f32 = 0.9;

                let delta_vector = obj.get_position() - CONSTRAINT_CENTER;

                if delta_vector.norm() > CONSTRAINT_RADIUS {
                    obj.set_position(
                        CONSTRAINT_CENTER + CONSTRAINT_RADIUS * delta_vector.normalize(),
                    );
                }
            }
            Constraint::Rectangular => {
                const CONSTRAINT_BOUND: f32 = 0.9;

                obj.set_position(Vec2::new(
                    obj.get_position()
                        .x
                        .clamp(-CONSTRAINT_BOUND, CONSTRAINT_BOUND),
                    obj.get_position()
                        .y
                        .clamp(-CONSTRAINT_BOUND, CONSTRAINT_BOUND),
                ));
            }
        }
    }

    pub fn update(&mut self, dt: f32, substeps: usize) {
        let dt = dt / substeps as f32;
        for _ in 0..substeps {
            self.apply_gravity();

            // TODO: determine the correct order of these two
            self.apply_constraint(&Constraint::Circular);
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

    pub fn add_obj_at(&mut self, position: Vec2) {
        let new_obj = VerletObject::new(position, RADIUS, self.renderer.default_shape.color);

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

    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    fn get_ij(x: f32, y: f32) -> (usize, usize) {
        (((x + 1.0) / RADIUS) as usize, ((y + 1.0) / RADIUS) as usize)
    }

    pub fn solve_collisions_with_grid(&mut self) {
        self.grid.iter_mut().for_each(Vec::clear);
        for (idx, obj) in self.objects.iter().enumerate() {
            let (x, y) = Self::get_ij(obj.get_position().x, obj.get_position().y);
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
                    for (pos_1, &obj_index_1) in possible_collisions.iter().enumerate() {
                        for &obj_index_2 in possible_collisions.iter().skip(pos_1 + 1) {
                            let obj_1 = &self.objects[obj_index_1];
                            let obj_2 = &self.objects[obj_index_2];

                            if obj_1.collides_with(obj_2) {
                                self.solve_collision(obj_index_1, obj_index_2);
                            }
                        }
                    }
                }
            }
        }
    }

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
        let delta_vector =
            self.objects[obj1_idx].get_position() - self.objects[obj2_idx].get_position();
        let distance = delta_vector.norm();
        let delta = delta_vector.normalize() * (RADIUS - distance / 2.0);
        self.objects[obj1_idx].shift(delta);
        self.objects[obj2_idx].shift(-delta);
    }

    pub fn update_vertex_buffer(&mut self) {
        self.renderer.update_vertex_buffer(&self.objects);
    }

    pub fn render(&self) {
        self.renderer.render();
    }
}

#[allow(dead_code)]
enum Constraint {
    Circular,
    Rectangular,
}
