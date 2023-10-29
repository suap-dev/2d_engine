use nalgebra_glm::Vec2;

const VEC2_ZERO: Vec2 = Vec2::new(0.0, 0.0);

pub struct VerletObject {
    position: Vec2,
    previous_position: Vec2,
    radius: f32,
    acceleration: Vec2,
    color: [f32; 4],
}

impl VerletObject {
    pub const fn new(position: Vec2, radius: f32, color: [f32; 4]) -> Self {
        Self {
            position,
            radius,
            previous_position: position,
            acceleration: VEC2_ZERO,
            color,
        }
    }

    pub const fn get_center(&self) -> Vec2 {
        self.position
    }

    pub fn shift(&mut self, vector: Vec2) {
        self.position += vector;
    }

    /// Forcefuly adjusts both ``position`` and ``previous_position`` to avoid changing velocity.
    /// This is works around the solver. Consider using ``fn shift(&mut self, vector: Vec)``
    /// if you're not sure you need this one.
    pub fn adjust_position_data(&mut self, vector: Vec2) {
        self.position += vector;
        // self.previous_position = self.position;
        self.previous_position += vector;
    }

    pub fn update_position(&mut self, dt: f32) {
        let velocity_dt = self.position - self.previous_position;
        self.previous_position = self.position;

        self.position = self.position + velocity_dt + self.acceleration * dt * dt;
        self.acceleration = VEC2_ZERO;
    }

    pub fn collides_with(&self, other: &Self) -> bool {
        self.position.metric_distance(&other.position) < self.radius + other.radius
    }

    pub fn set_acceleration(&mut self, acceleration: Vec2) {
        self.acceleration = acceleration;
    }

    pub fn accelerate(&mut self, delta: Vec2) {
        self.acceleration += delta;
    }

    pub const fn get_radius(&self) -> f32 {
        self.radius
    }

    pub const fn get_color(&self) -> [f32; 4] {
        self.color
    }
}
