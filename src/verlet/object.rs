use nalgebra_glm::Vec2;

const VEC2_ZERO: Vec2 = Vec2::new(0.0, 0.0);

#[derive(Default)]
pub struct VerletObject {
    position: VerletPosition,
    acceleration: Vec2,
}
impl VerletObject {
    pub fn update_position(&mut self, dt: f32) {
        self.position.apply_acceleration(self.acceleration, dt);
        self.acceleration = VEC2_ZERO;
    }

    pub fn add_acceleration(&mut self, acceleration: Vec2) {
        self.acceleration += acceleration;
    }
}

#[derive(Default)]
struct VerletPosition {
    previous: Vec2,
    current: Vec2,
}
impl VerletPosition {
    pub fn apply_acceleration(&mut self, acceleration: Vec2, dt: f32) {
        self.shift(self.delta() + acceleration * dt.powi(2));
    }

    fn delta(&self) -> Vec2 {
        self.previous - self.current
    }

    fn shift(&mut self, vector: Vec2) {
        self.previous = self.current;
        self.current += vector;
    }
}
