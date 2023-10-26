use std::time::Duration;

use nalgebra_glm::Vec2;

const VEC2_ZERO: Vec2 = Vec2::new(0.0, 0.0);

pub struct Entity {
    pub position: Vec2,
    previous_position: Vec2,
    radius: f32,
    pub acceleration: Vec2,
    pub color: [f32; 4],
}

impl Entity {
    pub const fn new(position: Vec2, radius: f32, color: [f32; 4]) -> Self {
        Self {
            position,
            radius,
            previous_position: position,
            acceleration: VEC2_ZERO,
            color,
        }
    }
    pub fn update_position(&mut self, dt: Duration) {
        let delta_position = self.position - self.previous_position;
        let dt = dt.as_secs_f32();
        self.previous_position = self.position;

        self.position = self.position + delta_position + self.acceleration * dt * dt;
        self.acceleration = VEC2_ZERO;

        self.apply_constraints(&Constraint::Rectangular);
    }
    // TODO: this should rather be in world.rs somehow
    // TODO: this is not taken into account by verlet solver... I think... double check it!
    fn apply_constraints(&mut self, constraint: &Constraint) {
        match constraint {
            Constraint::Circular => {
                const CONSTRAINT_CENTER: Vec2 = VEC2_ZERO;
                const CONSTRAINT_RADIUS: f32 = 0.9;

                let delta_vector = self.position - CONSTRAINT_CENTER;

                if delta_vector.norm() > CONSTRAINT_RADIUS {
                    self.position =
                        CONSTRAINT_CENTER + CONSTRAINT_RADIUS * delta_vector.normalize();
                }
            }
            Constraint::Rectangular => {
                const CONSTRAINT_BOUND: f32 = 0.9;

                self.position.x = self.position.x.clamp(-CONSTRAINT_BOUND, CONSTRAINT_BOUND);
                self.position.y = self.position.y.clamp(-CONSTRAINT_BOUND, CONSTRAINT_BOUND);
            }
        }
    }

    pub fn collides_with(&self, other: &Self) -> bool {
        self.position.metric_distance(&other.position) < self.radius + other.radius
    }
}

#[allow(dead_code)]
enum Constraint {
    Circular,
    Rectangular,
}
