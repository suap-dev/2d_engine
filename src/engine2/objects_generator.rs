use std::f32::consts::TAU;

use nalgebra_glm::Vec2;
use rand::random;

pub struct ObjectsGenerator {
    pub grid_center: Vec2,
    pub grid_columns: usize,
    pub grid_rows: usize,
    pub grid_rotation: f32,
    pub obj_radius: f32,
    pub obj_radius_deviation: f32,
    pub obj_min_separation: f32,
}
impl ObjectsGenerator {
    pub fn default() -> Self {
        Self {
            grid_center: Vec2::new(0.0, 0.0),
            grid_columns: 50,
            grid_rows: 80,
            grid_rotation: TAU / 45.0,
            obj_radius: 0.003,
            obj_radius_deviation: 0.0015,
            obj_min_separation: 0.0003,
        }
    }
    pub fn random_radius(&mut self) -> f32 {
        let randomizer = random::<f32>().mul_add(2.0, -1.0);
        let delta_radius = randomizer * self.obj_radius_deviation;
        self.obj_radius + delta_radius
    }
}
