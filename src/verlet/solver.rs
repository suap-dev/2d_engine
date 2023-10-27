use nalgebra_glm::Vec2;

use super::object::VerletObject;

struct Solver {
    gravity: Vec2,
    objects: Vec<VerletObject>,
}
impl Solver {
    fn new(gravity: Vec2) -> Self {
        Self {
            gravity,
            objects: Vec::default(),
        }
    }

    fn update(&mut self, dt: f32) {
        // TODO: can these be done simultaneously? do I need 2 iterations?
        self.apply_gravity();
        self.uprade_positions(dt);
    }

    fn uprade_positions(&mut self, dt: f32) {
        self.objects
            .iter_mut()
            .for_each(|object| object.update_position(dt));
    }

    fn apply_gravity(&mut self) {
        self.objects
            .iter_mut()
            .for_each(|object| object.add_acceleration(self.gravity));
    }
}
impl Default for Solver {
    fn default() -> Self {
        Self {
            gravity: Vec2::new(0.0, 1000.0),
            objects: Vec::default(),
        }
    }
}
