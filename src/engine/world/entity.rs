use std::time::Duration;

use nalgebra_glm::Vec2;

pub struct Entity {
    pub position: Vec2,
    previous_position: Vec2,
    radius: f32,
    pub acceleration: Vec2,
    pub color: [f32; 4],
}

impl Entity {
    pub fn new(position: Vec2, radius: f32, color: [f32; 4]) -> Self {
        Self {
            position,
            radius,
            previous_position: position,
            acceleration: Vec2::zeros(),
            color,
        }
    }
    pub fn update_position(&mut self, dt: Duration) {
        let delta_position = self.position - self.previous_position;
        let dt = dt.as_secs_f32();
        self.previous_position = self.position;

        self.position = self.position + delta_position + self.acceleration * dt * dt;
        self.acceleration.fill(0.0);

        self.apply_constraints(1);
    }
    // TODO: this should rather be in world.rs somehow
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

    pub fn collides_with(&self, other: &Self) -> bool {
        self.position.metric_distance(&other.position) < self.radius + other.radius
    }
}
